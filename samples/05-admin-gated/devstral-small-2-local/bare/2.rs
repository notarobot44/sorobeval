#![no_std]

use soroban_sdk::{contracterror, contractimpl, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[soroban_sdk::contract]
pub struct Gated;

#[contractimpl]
impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let admin = env
            .storage()
            .instance()
            .get::<Address>()
            .ok_or(Error::NotInitialized)?;
        env.invoke_function(
            id!(require_auth),
            soroban_sdk::symbol!("require_auth"),
            vec![(admin,)],
        );
        env.storage().instance().set(&value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        env.storage()
            .instance()
            .get::<u32>()
            .unwrap_or_default()
    }
}
