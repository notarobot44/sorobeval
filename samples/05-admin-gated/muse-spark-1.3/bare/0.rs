#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct Gated;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

const ADMIN_KEY: Symbol = symbol_short!("admin");
const VALUE_KEY: Symbol = symbol_short!("value");

#[contractimpl]
impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&ADMIN_KEY, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN_KEY)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage().instance().set(&VALUE_KEY, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        env.storage().instance().get(&VALUE_KEY).unwrap_or(0)
    }
}
