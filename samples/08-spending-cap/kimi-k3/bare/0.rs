#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, token, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&"admin") {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        env.storage().instance().set(&"admin", &admin);
        env.storage().instance().set(&"token", &token);
        env.storage().instance().set(&"cap", &cap);
        env.storage().instance().set(&"spent", &0i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&"admin")
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&("wl", recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&"admin")
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let whitelisted: bool = env
            .storage()
            .persistent()
            .get(&("wl", to.clone()))
            .unwrap_or(false);
        if !whitelisted {
            return Err(Error::NotWhitelisted);
        }

        let cap: i128 = env.storage().instance().get(&"cap").unwrap_or(0);
        let spent: i128 = env.storage().instance().get(&"spent").unwrap_or(0);
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        let token_addr: Address = env
            .storage()
            .instance()
            .get(&"token")
            .ok_or(Error::NotInitialized)?;
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &to, &amount);

        env.storage().instance().set(&"spent", &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&"spent").unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&"cap").unwrap_or(0)
    }
}
