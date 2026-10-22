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
        if env.storage().instance().has(&symbol_short!("ADMIN")) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        env.storage().instance().set(&symbol_short!("ADMIN"), &admin);
        env.storage().instance().set(&symbol_short!("TOKEN"), &token);
        env.storage().instance().set(&symbol_short!("CAP"), &cap);
        env.storage().instance().set(&symbol_short!("SPENT"), &0i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&(symbol_short!("WL"), recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !env
            .storage()
            .persistent()
            .has(&(symbol_short!("WL"), to.clone()))
        {
            return Err(Error::NotWhitelisted);
        }
        let spent: i128 = env
            .storage()
            .instance()
            .get(&symbol_short!("SPENT"))
            .unwrap_or(0);
        let cap: i128 = env
            .storage()
            .instance()
            .get(&symbol_short!("CAP"))
            .ok_or(Error::NotInitialized)?;
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }
        let token_addr: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("TOKEN"))
            .ok_or(Error::NotInitialized)?;
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &to, &amount);
        env.storage()
            .instance()
            .set(&symbol_short!("SPENT"), &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&symbol_short!("SPENT"))
            .unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&symbol_short!("CAP"))
            .unwrap_or(0)
    }

    fn admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&symbol_short!("ADMIN"))
            .ok_or(Error::NotInitialized)
    }
}
