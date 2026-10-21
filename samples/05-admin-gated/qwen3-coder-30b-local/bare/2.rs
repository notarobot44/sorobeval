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
        let existing_admin = env.storage().get::<_, Address>(StorageType::Instance, &"admin");
        if existing_admin.is_some() {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().set::<Address>(StorageType::Instance, &"admin", &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let admin = env.storage().get::<_, Address>(StorageType::Instance, &"admin")
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage().set::<u32>(StorageType::Instance, &"value", &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        env.storage()
            .get::<_, u32>(StorageType::Instance, &"value")
            .unwrap_or(0)
    }
}
