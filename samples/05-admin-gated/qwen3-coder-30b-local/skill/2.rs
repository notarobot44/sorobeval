#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        let admin_key = Symbol::new(&env, "admin");
        if env.storage().instance().has(&admin_key) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&admin_key, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let admin_key = Symbol::new(&env, "admin");
        let admin: Address = env.storage().instance().get(&admin_key).ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage().instance().set(&Symbol::new(&env, "value"), &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        let value_key = Symbol::new(&env, "value");
        env.storage().instance().get(&value_key).unwrap_or(0)
    }
}
