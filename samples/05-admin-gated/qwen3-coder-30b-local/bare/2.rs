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
        let initialized = env.storage().has(&StorageType::Instance, b"admin");
        if initialized {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().set(&StorageType::Instance, b"admin", &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let admin = env.storage().get::<Address>(&StorageType::Instance, b"admin")
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage().set(&StorageType::Instance, b"value", &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        env.storage().get(&StorageType::Instance, b"value").unwrap_or(0)
    }
}
