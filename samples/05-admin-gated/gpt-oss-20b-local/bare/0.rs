#![no_std]

use soroban_sdk::{
    contractimpl,
    contracterror,
    Env,
    Address,
    Symbol,
    Vec,
};

#[contracterror]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

const ADMIN_KEY: Symbol = Symbol::short("ADMIN");
const VALUE_KEY: Symbol = Symbol::short("VALUE");

pub struct Gated;

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

        // Ensure contract has been initialized
        if !storage.has(&ADMIN_KEY) {
            return Err(Error::NotInitialized);
        }

        // Retrieve admin and require authorization
        let admin: Address = storage.get(&ADMIN_KEY).unwrap();
        env.authorize_asset(&admin);

        // Store the value
        storage.set(&VALUE_KEY, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        let storage = env.storage().instance();
        if let Some(v): Option<u32> = storage.get(&VALUE_KEY) {
            v.unwrap()
        } else {
            0
        }
    }
}
