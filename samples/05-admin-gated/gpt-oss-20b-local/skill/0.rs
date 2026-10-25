#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, Address, Env,
};

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
    pub fn init(env: &Env, admin: Address) -> Result<(), Error> {
        // Check if already initialized
        let key_admin = soroban_sdk::symbol_short!("admin");
        if env.storage().instance().has(&key_admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&key_admin, &admin);
        Ok(())
    }

    pub fn set_value(env: &Env, value: u32) -> Result<(), Error> {
        let key_admin = soroban_sdk::symbol_short!("admin");
        // Ensure init was called
        let admin: Address = env
            .storage()
            .instance()
            .get(&key_admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        let key_value = soroban_sdk::symbol_short!("value");
        env.storage().instance().set(&key_value, &value);
        Ok(())
    }

    pub fn value(env: &Env) -> u32 {
        let key_value = soroban_sdk::symbol_short!("value");
        env.storage()
            .instance()
            .get(&key_value)
            .unwrap_or(0u32)
    }
}
