#![no_std]
extern crate soroban_sdk;

use soroban_sdk::{contractimpl, contracttype, Address, Env, Symbol};

#[derive(Clone)]
#[contracttype]
struct Key(&'static str);

const ADMIN_KEY: Key = Key("admin");
const VALUE_KEY: Key = Key("value");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Error {
    #[msg("Contract already initialized")]
    AlreadyInitialized = 1,
    #[msg("Contract not initialized")]
    NotInitialized = 2,
}
impl soroban_sdk::contracterror for Error {}

pub struct Gated;

#[contractimpl]
impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let storage = env.storage();
        if storage.has(&ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        storage.set(&ADMIN_KEY, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let storage = env.storage();
        let admin: Address = match storage.get(&ADMIN_KEY) {
            Some(a) => a,
            None => return Err(Error::NotInitialized),
        };
        env.authorize(&admin);
        storage.set(&VALUE_KEY, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        let storage = env.storage();
        match storage.get::<_, u32>(&VALUE_KEY) {
            Some(v) => v,
            None => 0,
        }
    }
}
