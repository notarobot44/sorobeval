#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[derive(Clone)]
enum DataKey {
    Admin,
    Value,
}

#[contract]
pub struct Gated;

#[contractimpl]
impl Gated {
    /// Initializes the contract with an admin. Can only be called once.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let instance = env.storage().instance();
        if instance.has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        instance.set(&DataKey::Admin, &admin);
        // Default value is 0; no need to store explicitly.
        Ok(())
    }

    /// Sets the stored value. Requires admin authorization.
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

    /// Returns the stored value, 0 if never set.
    pub fn value(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Value)
            .unwrap_or(0u32)
    }
}
