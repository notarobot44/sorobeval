#![no_std]

extern crate soroban_sdk;

use soroban_sdk::{contractimpl, Env, Address, Symbol};

#[derive(Debug, Clone)]
pub enum Error {
    #[soroban_error]
    AlreadyInitialized,
    #[soroban_error]
    NotInitialized,
}

#[contractimpl]
pub struct Gated;

const ADMIN_KEY: Symbol = Symbol::short("admin");
const VALUE_KEY: Symbol = Symbol::short("value");

impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        // Check if already initialized
        if env.storage().instance().has(&ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        // Store the admin address
        env.storage()
            .instance()
            .set(&ADMIN_KEY, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        // Retrieve stored admin; error if not initialized
        let admin = match env.storage().instance().get::<Address>(&ADMIN_KEY) {
            Some(a) => a,
            None => return Err(Error::NotInitialized),
        };

        // Require authorization from the admin
        env.require_auth(&admin);

        // Store the new value
        env.storage()
            .instance()
            .set(&VALUE_KEY, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        env.storage()
            .instance()
            .get::<u32>(&VALUE_KEY)
            .unwrap_or_default()
    }
}
