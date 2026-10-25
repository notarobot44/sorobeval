#![no_std]

use soroban_sdk::{
    contractimpl, contracterror, Env, Address, BytesN, Symbol,
};

#[contracterror]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

pub struct Gated;

const ADMIN_KEY: Symbol = Symbol::short("admin");
const VALUE_KEY: Symbol = Symbol::short("value");

#[contractimpl]
impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let storage = env.storage().instance();
        if storage.has(&ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        storage.set(&ADMIN_KEY, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let storage = env.storage().instance();

        // Ensure initialized
        let admin: Address = match storage.get(&ADMIN_KEY) {
            Some(a) => a,
            None => return Err(Error::NotInitialized),
        };

        // Require admin authorization
        env.require_auth(&admin);

        // Store the value
        storage.set(&VALUE_KEY, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        let storage = env.storage().instance();
        match storage.get::<_, u32>(&VALUE_KEY) {
            Some(v) => v,
            None => 0,
        }
    }
}
