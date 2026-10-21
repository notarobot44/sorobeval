#![no_std]

use soroban_sdk::{contractimpl, contracterror, Env, Address};

#[derive(Clone)]
pub struct Gated;

#[contracterror]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[contractimpl]
impl Gated {
    const ADMIN_KEY: &[u8] = b"admin";
    const VALUE_KEY: &[u8] = b"value";

    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let instance = env.storage().instance();
        if instance.has(&Self::ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        instance.set(&Self::ADMIN_KEY, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let instance = env.storage().instance();
        let admin: Address = match instance.get::<Address>(&Self::ADMIN_KEY) {
            Some(a) => a,
            None => return Err(Error::NotInitialized),
        };
        admin.require_auth();

        instance.set(&Self::VALUE_KEY, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        let instance = env.storage().instance();
        match instance.get::<u32>(&Self::VALUE_KEY) {
            Some(v) => v,
            None => 0,
        }
    }
}
