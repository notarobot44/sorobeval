#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Value,
}

#[contract]
pub struct Gated;

#[contractimpl]
impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage().instance().set(&DataKey::Value, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        env.storage()
            .instance()
            .get::<DataKey, u32>(&DataKey::Value)
            .unwrap_or(0)
    }
}
