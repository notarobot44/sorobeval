#![no_std]

use soroban_sdk::{
    contract::{contractimpl, Env},
    token::Client,
    Address, BytesN, Symbol, Vec, IntoVal,
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

const ADMIN_KEY: Symbol = Symbol::short("admin");
const TOKEN_KEY: Symbol = Symbol::short("token");
const CAP_KEY: Symbol = Symbol::short("cap");
const SPENT_KEY: Symbol = Symbol::short("spent");
const WHITELIST_PREFIX: &str = "whitelist_";

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        // Ensure not already initialized
        if env.storage().has(&ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        env.storage().set(&ADMIN_KEY, &admin.clone());
        env.storage().set(&TOKEN_KEY, &token.clone());
        env.storage().set(&CAP_KEY, &cap);
        // spent starts at zero
        env.storage().set(&SPENT_KEY, &0i128);

        Ok(())
    }

    fn require_admin(env: &Env) -> Result<(), Error> {
        let admin: Address = env.storage().get(&ADMIN_KEY).ok_or(Error::NotInitialized)?;
        env.require_auth(admin);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        Self::require_admin(&env)?;

        let key = BytesN::<32>::from_slice(
            format!("{}{}", WHITELIST_PREFIX, recipient.as_bytes()).as_bytes(),
        );
        env.storage().set(&key, &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        Self::require_admin(&env)?;

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let key = BytesN::<32>::from_slice(
            format!("{}{}", WHITELIST_PREFIX, to.as_bytes()).as_bytes(),
        );
        if !env.storage().has(&key) {
            return Err(Error::NotWhitelisted);
        }

        let cap: i128 = env.storage().get(&CAP_KEY).ok_or(Error::NotInitialized)?;
        let spent: i128 = env
            .storage()
            .get(&SPENT_KEY)
            .unwrap_or_else(|| 0i128);

        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        // Perform transfer
        let token_addr: Address = env.storage().get(&TOKEN_KEY).ok_or(Error::NotInitialized)?;
        let token = Client::new(&env, &token_addr);
        token.transfer(
            &env.current_contract_address(),
            &to,
            amount,
        );

        // Update spent
        env.storage()
            .set(&SPENT_KEY, &(spent + amount));

        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().get(&SPENT_KEY).unwrap_or_else(|| 0i128)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage()
            .get(&CAP_KEY)
            .unwrap_or_else(|| 0i128)
    }
}
