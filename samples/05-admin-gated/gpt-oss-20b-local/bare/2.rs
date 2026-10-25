#![no_std]

use soroban_sdk::{contractimpl, contracterror, Env, Address};

#[derive(Clone)]
pub struct Gated;

#[contracterror]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

const ADMIN_KEY: &[u8] = b"admin";
const VALUE_KEY: &[u8] = b"value";

#[contractimpl]
impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let storage = env.storage().instance();
        if storage.has(ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        storage.set(ADMIN_KEY, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let storage = env.storage().instance();
        let admin: Option<Address> = storage.get(ADMIN_KEY);
        match admin {
            Some(admin_addr) => {
                env.require_auth(&admin_addr);
                storage.set(VALUE_KEY, &value);
                Ok(())
            }
            None => Err(Error::NotInitialized),
        }
    }

    pub fn value(env: Env) -> u32 {
        let storage = env.storage().instance();
        storage.get(VALUE_KEY).unwrap_or(0)
    }
}
