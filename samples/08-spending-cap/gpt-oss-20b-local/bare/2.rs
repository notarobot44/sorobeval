#![no_std]

use soroban_sdk::{
    contractimpl, contracttype, Env, Address, Symbol, IntoVal,
    token::Client as TokenClient,
};

#[derive(Clone)]
#[contracttype]
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

#[contractimpl]
impl Treasury {
    // Storage keys
    const ADMIN: Symbol = Symbol::short("ADMIN");
    const TOKEN: Symbol = Symbol::short("TOKEN");
    const CAP: Symbol = Symbol::short("CAP");
    const SPENT: Symbol = Symbol::short("SPENT");
    const WHITELIST_PREFIX: Symbol = Symbol::short("WL");

    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().has(&Self::ADMIN) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        env.storage().set(&Self::ADMIN, &admin);
        env.storage().set(&Self::TOKEN, &token);
        env.storage().set(&Self::CAP, &cap);
        env.storage().set(&Self::SPENT, &i128::from(0));
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = env.storage().get::<_, Address>(&Self::ADMIN).ok_or(Error::NotInitialized)?;
        env.require_auth(&admin);

        // store a marker value (true)
        env.storage()
            .set(&(Self::WHITELIST_PREFIX, recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let admin = env.storage().get::<_, Address>(&Self::ADMIN).ok_or(Error::NotInitialized)?;
        env.require_auth(&admin);

        // check whitelist
        let is_whitelisted: Option<bool> = env
            .storage()
            .get(&(Self::WHITELIST_PREFIX, to));
        if !is_whitelisted.unwrap_or(false) {
            return Err(Error::NotWhitelisted);
        }

        // cap check
        let cap: i128 = env.storage().get(&Self::CAP).unwrap();
        let spent: i128 = env.storage().get(&Self::SPENT).unwrap();
        let new_total = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_total > cap {
            return Err(Error::CapExceeded);
        }

        // transfer tokens
        let token_addr: Address = env.storage().get(&Self::TOKEN).unwrap();
        let token_client = TokenClient::new(&env, &token_addr);
        let contract_addr = env.current_contract_address();
        token_client.transfer(
            &contract_addr,
            &to,
            &amount,
            &Symbol::short(""),
            &[],
        );

        // update spent
        env.storage().set(&Self::SPENT, &new_total);
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage()
            .get::<_, i128>(&Self::SPENT)
            .unwrap_or_else(|| i128::from(0))
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().get(&Self::CAP).unwrap_or_else(|| i128::from(0))
    }
}
