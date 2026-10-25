#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, Address, Env,
};

#[contracterror]
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[contract]
pub struct Gated;

#[contractimpl]
impl Gated {
    // Initializes the contract with an admin. Fails if already initialized.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let key_admin = DataKey::Admin;
        if env.storage().instance().has(&key_admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&key_admin, &admin);
        Ok(())
    }

    // Sets a value. Requires the stored admin to authorize.
    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let key_admin = DataKey::Admin;
        let admin: Address = env
            .storage()
            .instance()
            .get(&key_admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        let key_value = DataKey::Value;
        env.storage().instance().set(&key_value, &value);
        Ok(())
    }

    // Retrieves the stored value; 0 if never set.
    pub fn value(env: Env) -> u32 {
        let key_value = DataKey::Value;
        env.storage()
            .instance()
            .get(&key_value)
            .unwrap_or(0u32)
    }
}

// Typed storage keys
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Value,
}
