#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Address, Env};

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

const ADMIN: soroban_sdk::Symbol = symbol_short!("ADMIN");
const TOKEN: soroban_sdk::Symbol = symbol_short!("TOKEN");
const CAP: soroban_sdk::Symbol = symbol_short!("CAP");
const SPENT: soroban_sdk::Symbol = symbol_short!("SPENT");

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOKEN, &token);
        env.storage().instance().set(&CAP, &cap);
        env.storage().instance().set(&SPENT, &0i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        env.storage().persistent().set(&recipient, &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin = Self::admin(&env)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !env.storage().persistent().has(&to) {
            return Err(Error::NotWhitelisted);
        }

        let cap: i128 = env.storage().instance().get(&CAP).ok_or(Error::NotInitialized)?;
        let spent: i128 = env.storage().instance().get(&SPENT).unwrap_or(0);
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        let token: Address = env.storage().instance().get(&TOKEN).ok_or(Error::NotInitialized)?;
        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &to, &amount);

        env.storage().instance().set(&SPENT, &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&SPENT).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&CAP).unwrap_or(0)
    }

    fn admin(env: &Env) -> Result<Address, Error> {
        env.storage().instance().get(&ADMIN).ok_or(Error::NotInitialized)
    }
}
