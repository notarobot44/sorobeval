#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, Address, Env,
};

#[contract]
pub struct Gated;

#[derive(Clone)]
enum StorageKey {
    Admin,
    Value,
}

#[contracterror]
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[contractimpl]
impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        // If an admin is already set, reject.
        if env
            .storage()
            .instance()
            .has(&StorageKey::Admin)
        {
            return Err(Error::AlreadyInitialized);
        }
        // Store the new admin.
        env.storage().instance().set(&StorageKey::Admin, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        // Retrieve stored admin; if not present, init hasn't run.
        let admin: Address = match env
            .storage()
            .instance()
            .get(&StorageKey::Admin)
        {
            Some(a) => a,
            None => return Err(Error::NotInitialized),
        };

        // Require the admin's authorization for this operation.
        admin.require_auth();

        // Persist the new value.
        env.storage().instance().set(&StorageKey::Value, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&StorageKey::Value)
            .unwrap_or(0u32)
    }
}
