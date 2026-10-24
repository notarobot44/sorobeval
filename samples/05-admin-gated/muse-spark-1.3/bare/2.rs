#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Address, Env, Symbol};

const ADMIN: Symbol = symbol_short!("ADMIN");
const VALUE: Symbol = symbol_short!("VALUE");

#[contract]
pub struct Gated;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[contractimpl]
impl Gated {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&ADMIN, &admin);
        Ok(())
    }

    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let admin: Option<Address> = env.storage().instance().get(&ADMIN);
        let admin = match admin {
            Some(a) => a,
            None => return Err(Error::NotInitialized),
        };
        admin.require_auth();
        env.storage().instance().set(&VALUE, &value);
        Ok(())
    }

    pub fn value(env: Env) -> u32 {
        env.storage().instance().get::<Symbol, u32>(&VALUE).unwrap_or(0)
    }
}
