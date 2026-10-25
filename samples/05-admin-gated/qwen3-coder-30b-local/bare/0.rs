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
        if env.storage().has(&StorageType::Instance, b"admin") {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().set(&StorageType::Instance, b"admin", &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        if !env.storage().has(&StorageType::Instance, b"admin") {
            return Err(Error::NotInitialized);
        }
        env.require_auth(&env.storage().get::<Address>(&StorageType::Instance, b"admin").unwrap());
        env.storage().set(&StorageType::Instance, b"value", &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        match env.storage().get::<u32>(&StorageType::Instance, b"value") {
            Some(v) => v,
            None => 0,
        }
    }
}
