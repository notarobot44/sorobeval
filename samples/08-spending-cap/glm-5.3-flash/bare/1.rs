#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, Address, Env, Map};

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contract]
pub struct Treasury;

const ADMIN: &str = "admin";
const TOKEN: &str = "token";
const CAP: &str = "cap";
const SPENT: &str = "spent";
const WHITELIST: &str = "whitelist";

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        if env.storage().persistent().has(&ADMIN) {
            return Err(Error::AlreadyInitialized);
        }
        let mut storage = env.storage().persistent();
        storage.set(&ADMIN, &admin);
        storage.set(&TOKEN, &token);
        storage.set(&CAP, &cap);
        storage.set(&SPENT, &0i128);
        storage.set(&WHITELIST, &Map::<Address, bool>::new(&env));
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        Self::require_init(&env)?;
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();
        let mut wl: Map<Address, bool> = env.storage().persistent().get(&WHITELIST).unwrap();
        wl.set(recipient, true);
        env.storage().persistent().set(&WHITELIST, &wl);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        Self::require_init(&env)?;
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();
        let wl: Map<Address, bool> = env.storage().persistent().get(&WHITELIST).unwrap();
        if !wl.contains_key(&to) {
            return Err(Error::NotWhitelisted);
        }
        let storage = env.storage().persistent();
        let cap: i128 = storage.get(&CAP).unwrap();
        let spent: i128 = storage.get(&SPENT).unwrap();
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }
        let token: Address = storage.get(&TOKEN).unwrap();
        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &to, &amount);
        storage.set(&SPENT, &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().persistent().get(&SPENT).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().persistent().get(&CAP).unwrap_or(0)
    }

    fn require_init(env: &Env) -> Result<(), Error> {
        if env.storage().persistent().has(&ADMIN) {
            Ok(())
        } else {
            Err(Error::NotInitialized)
        }
    }
}
