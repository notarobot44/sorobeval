#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, env::Env, Address, StorageType};

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
        let key = &"admin";
        if env.storage().has(key, StorageType::Instance) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().set(key, StorageType::Instance, admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let key = &"admin";
        if !env.storage().has(key, StorageType::Instance) {
            return Err(Error::NotInitialized);
        }
        let admin: Address = env.storage().get(key, StorageType::Instance).unwrap();
        admin.require_auth();
        env.storage().set(&"value", StorageType::Instance, value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        let key = &"value";
        if env.storage().has(key, StorageType::Instance) {
            env.storage().get(key, StorageType::Instance).unwrap()
        } else {
            0
        }
    }
}
