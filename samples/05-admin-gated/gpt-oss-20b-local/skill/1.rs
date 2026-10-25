#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
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

#[contract]
pub struct Gated;

#[contractimpl]
impl Gated {
    /// Initializes the contract with an admin. Fails if already initialized.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let instance = env.storage().instance();
        if instance.has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        instance.set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Sets a value. Requires the stored admin's authorization.
    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let instance = env.storage().instance();
        let admin: Address = match instance.get(&DataKey::Admin) {
            Some(a) => a,
            None => return Err(Error::NotInitialized),
        };
        admin.require_auth();

        instance.set(&DataKey::Value, &value);
        Ok(())
    }

    /// Returns the stored value, or 0 if never set.
    pub fn value(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Value)
            .unwrap_or(0u32)
    }
}
