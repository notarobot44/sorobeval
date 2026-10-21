#![no_std]

use soroban_sdk::{
    contractimpl, contracttype, Env, Address, BytesN, Symbol, map::Map,
};

#[derive(Clone)]
pub struct Treasury;

#[contracterror]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contracttype]
struct Key(String);

impl Treasury {
    // Storage keys
    const ADMIN_KEY: Symbol = Symbol::short("ADMIN");
    const TOKEN_KEY: Symbol = Symbol::short("TOKEN");
    const CAP_KEY: Symbol = Symbol::short("CAP");
    const SPENT_KEY: Symbol = Symbol::short("SPENT");

    fn get_admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .get(&Self::ADMIN_KEY)
            .ok_or(Error::NotInitialized)
    }

    fn require_auth_admin(env: &Env) -> Result<(), Error> {
        let admin = Self::get_admin(env)?;
        if !env.invocation().has_auth(&admin) {
            Err(Error::NotInitialized)
        } else {
            Ok(())
        }
    }

    #[allow(dead_code)]
    fn get_token(env: &Env) -> Result<Address, Error> {
        env.storage()
            .get(&Self::TOKEN_KEY)
            .ok_or(Error::NotInitialized)
    }

    #[allow(dead_code)]
    fn get_cap(env: &Env) -> Result<i128, Error> {
        env.storage()
            .get(&Self::CAP_KEY)
            .ok_or(Error::NotInitialized)
    }

    #[allow(dead_code)]
    fn get_spent(env: &Env) -> Result<i128, Error> {
        env.storage()
            .get(&Self::SPENT_KEY)
            .ok_or(Error::NotInitialized)
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
        if env.storage().has(&Self::ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        env.storage().set(&Self::ADMIN_KEY, &admin);
        env.storage().set(&Self::TOKEN_KEY, &token);
        env.storage().set(&Self::CAP_KEY, &cap);
        env.storage().set(&Self::SPENT_KEY, &0i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        Self::require_auth_admin(&env)?;
        let key = Key(format!("whitelist_{}", recipient.as_str()));
        env.storage().set(&key, &true);
        Ok(())
    }

    pub fn spend(
        env: Env,
        to: Address,
        amount: i128,
    ) -> Result<(), Error> {
        Self::require_auth_admin(&env)?;

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let key = Key(format!("whitelist_{}", to.as_str()));
        if !env.storage().has(&key) {
            return Err(Error::NotWhitelisted);
        }

        let cap: i128 = env
            .storage()
            .get(&Self::CAP_KEY)
            .ok_or(Error::NotInitialized)?;
        let spent: i128 = env
            .storage()
            .get(&Self::SPENT_KEY)
            .ok_or(Error::NotInitialized)?;

        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        // Transfer tokens from contract to recipient
        let token_addr: Address = env
            .storage()
            .get(&Self::TOKEN_KEY)
            .ok_or(Error::NotInitialized)?;
        let client = soroban_sdk::token::Client::new(&env, &token_addr);
        let contract_self = env.current_contract_address();
        client.transfer_from(
            &contract_self,
            &to,
            &amount,
        );

        // Update spent
        env.storage()
            .set(&Self::SPENT_KEY, &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage()
            .get(&Self::SPENT_KEY)
            .unwrap_or(0i128)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage()
            .get(&Self::CAP_KEY)
            .unwrap_or(0i128)
    }
}
