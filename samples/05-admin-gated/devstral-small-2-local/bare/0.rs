#![no_std]

use soroban_sdk::{contracterror, contractimpl, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[contractimpl]
pub struct Gated;

const ADMIN_KEY: &str = "admin";
const VALUE_KEY: &str = "value";

pub fn init(env: Env, admin: Address) -> Result<(), Error> {
    if env.storage().has(&ADMIN_KEY) {
        return Err(Error::AlreadyInitialized);
    }
    env.storage().persistent().set(&ADMIN_KEY, &admin);
    Ok(())
}

pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
    let admin = env
        .storage()
        .persistent()
        .get::<Address>(&ADMIN_KEY)
        .ok_or(Error::NotInitialized)?;
    env.invoke_function(
        (admin,),
        "require_auth",
        soroban_sdk::symbol!("require_auth"),
        &[],
    );
    env.storage().persistent().set(&VALUE_KEY, &value);
    Ok(())
}

pub fn value(env: Env) -> u32 {
    env.storage()
        .persistent()
        .get::<u32>(&VALUE_KEY)
        .unwrap_or(0)
}
