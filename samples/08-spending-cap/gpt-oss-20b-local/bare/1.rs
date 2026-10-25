#![no_std]

use soroban_sdk::{
    contract::{Contract, ContractClient},
    contracterror,
    token::Client as TokenClient,
    Address, Env, BytesN, Symbol, Vec,
};

/// Storage keys
const ADMIN_KEY: Symbol = Symbol::short("ADMIN");
const TOKEN_KEY: Symbol = Symbol::short("TOKEN");
const CAP_KEY: Symbol = Symbol::short("CAP");
const SPENT_KEY: Symbol = Symbol::short("SPENT");

#[contracterror]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[derive(Clone)]
pub struct Treasury;

impl Treasury {
    fn storage(env: &Env) -> soroban_sdk::storage::PersistentMap<BytesN<u8>, BytesN<u8>> {
        env.storage().persistent()
    }

    fn get_admin(env: &Env) -> Result<Address, Error> {
        let key = ADMIN_KEY;
        match Self::storage(env).get(&key.into()) {
            Some(bytes) => Ok(Address::from_bytes(bytes)),
            None => Err(Error::NotInitialized),
        }
    }

    fn set_admin(env: &Env, admin: Address) -> Result<(), Error> {
        let key = ADMIN_KEY;
        if Self::storage(env).contains_key(&key.into()) {
            return Err(Error::AlreadyInitialized);
        }
        Self::storage(env).set(&key.into(), &admin.to_bytes());
        Ok(())
    }

    fn get_token(env: &Env) -> Result<Address, Error> {
        let key = TOKEN_KEY;
        match Self::storage(env).get(&key.into()) {
            Some(bytes) => Ok(Address::from_bytes(bytes)),
            None => Err(Error::NotInitialized),
        }
    }

    fn set_token(env: &Env, token: Address) -> Result<(), Error> {
        let key = TOKEN_KEY;
        if Self::storage(env).contains_key(&key.into()) {
            return Err(Error::AlreadyInitialized);
        }
        Self::storage(env).set(&key.into(), &token.to_bytes());
        Ok(())
    }

    fn get_cap(env: &Env) -> Result<i128, Error> {
        let key = CAP_KEY;
        match Self::storage(env).get(&key.into()) {
            Some(bytes) => Ok(i128::from_le_bytes(
                BytesN::<u8>::from_bytes(bytes)
                    .unwrap_or_default()
                    .try_into()
                    .unwrap(),
            )),
            None => Err(Error::NotInitialized),
        }
    }

    fn set_cap(env: &Env, cap: i128) -> Result<(), Error> {
        let key = CAP_KEY;
        if Self::storage(env).contains_key(&key.into()) {
            return Err(Error::AlreadyInitialized);
        }
        let bytes = BytesN::<u8>::from_bytes(cap.to_le_bytes().to_vec());
        Self::storage(env).set(&key.into(), &bytes);
        Ok(())
    }

    fn get_spent(env: &Env) -> Result<i128, Error> {
        let key = SPENT_KEY;
        match Self::storage(env).get(&key.into()) {
            Some(bytes) => Ok(i128::from_le_bytes(
                BytesN::<u8>::from_bytes(bytes)
                    .unwrap_or_default()
                    .try_into()
                    .unwrap(),
            )),
            None => Ok(0),
        }
    }

    fn set_spent(env: &Env, amount: i128) -> Result<(), Error> {
        let key = SPENT_KEY;
        let bytes = BytesN::<u8>::from_bytes(amount.to_le_bytes().to_vec());
        Self::storage(env).set(&key.into(), &bytes);
        Ok(())
    }

    fn whitelist_key(recipient: &Address) -> BytesN<u8> {
        // Prefix "W" + recipient bytes
        let mut data = Vec::<u8>::new();
        data.push(b'W');
        data.extend_from_slice(&recipient.to_bytes());
        BytesN::from(data)
    }

    fn add_whitelist(env: &Env, recipient: Address) -> Result<(), Error> {
        let key = Self::whitelist_key(&recipient);
        Self::storage(env).set(&key.into(), &BytesN::<u8>::default()); // value unused
        Ok(())
    }

    fn is_whitelisted(env: &Env, recipient: &Address) -> bool {
        let key = Self::whitelist_key(recipient);
        Self::storage(env).contains_key(&key.into())
    }
}

#[contractimpl]
impl Treasury {
    pub fn init(
        env: Env,
        admin: Address,
        token: Address,
        cap: i128,
    ) -> Result<(), Error> {
        // Ensure not already initialized
        if Self::storage(&env).contains_key(&ADMIN_KEY.into()) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        Self::set_admin(&env, admin)?;
        Self::set_token(&env, token)?;
        Self::set_cap(&env, cap)?;
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = Self::get_admin(&env)?;
        env.auth().require_auth(admin.clone());
        Self::add_whitelist(&env, recipient)
    }

    pub fn spend(
        env: Env,
        to: Address,
        amount: i128,
    ) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let admin = Self::get_admin(&env)?;
        env.auth().require_auth(admin.clone());

        if !Self::is_whitelisted(&env, &to) {
            return Err(Error::NotWhitelisted);
        }

        let cap = Self::get_cap(&env)?;
        let spent = Self::get_spent(&env)?;
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        // Transfer tokens
        let token_addr = Self::get_token(&env)?;
        let client: TokenClient = TokenClient::new(token_addr.clone());
        client.transfer(
            &env,
            &token_addr, // from contract itself
            &to,
            amount,
        );

        Self::set_spent(&env, spent + amount)
    }

    pub fn spent(env: Env) -> Result<i128, Error> {
        Ok(Self::get_spent(&env)?)
    }

    pub fn cap(env: Env) -> Result<i128, Error> {
        Ok(Self::get_cap(&env)?)
    }
}
