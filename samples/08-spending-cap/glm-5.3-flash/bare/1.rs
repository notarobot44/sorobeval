#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Address, Env};

#[contracterror]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contract]
pub struct Treasury;

const ADMIN: symbol_short::Symbol = symbol_short!("admin");
const TOKEN: symbol_short::Symbol = symbol_short!("token");
const CAP: symbol_short::Symbol = symbol_short!("cap");
const SPENT: symbol_short::Symbol = symbol_short!("spent");
const WL: symbol_short::Symbol = symbol_short!("wl");

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().persistent().get(&ADMIN).is_some() {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        env.storage().persistent().set(&ADMIN, &admin);
        env.storage().persistent().set(&TOKEN, &token);
        env.storage().persistent().set(&CAP, &cap);
        env.storage().persistent().set(&SPENT, &0i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&(WL, recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !env.storage().persistent().get(&(WL, to)).unwrap_or(false) {
            return Err(Error::NotWhitelisted);
        }
        let cap: i128 = env.storage().persistent().get(&CAP).ok_or(Error::NotInitialized)?;
        let spent: i128 = env.storage().persistent().get(&SPENT).unwrap_or(0);
        if spent.saturating_add(amount) > cap {
            return Err(Error::CapExceeded);
        }

        let token: Address = env.storage().persistent().get(&TOKEN).ok_or(Error::NotInitialized)?;
        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &to, &amount);

        env.storage().persistent().set(&SPENT, &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().persistent().get(&SPENT).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().persistent().get(&CAP).unwrap_or(0)
    }
}
