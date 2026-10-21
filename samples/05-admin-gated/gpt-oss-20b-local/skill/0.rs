#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env,
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    Value,
}

#[contracterror]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[contract]
pub struct Gated;

#[contractimpl]
impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let storage = env.storage().instance();
        if storage.has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        storage.set(&DataKey::Admin, &admin);
        // Optionally initialize value to 0; not required by spec
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let storage = env.storage().instance();
        let admin: Address = match storage.get(&DataKey::Admin) {
            Some(a) => a,
            None => return Err(Error::NotInitialized),
        };
        admin.require_auth();

        storage.set(&DataKey::Value, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Value)
            .unwrap_or(0u32)
    }
}
