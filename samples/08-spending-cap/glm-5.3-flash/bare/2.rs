#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, Address, Env, Map};

#[contracterror]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contract]
pub struct Treasury;

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelist,
}

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        env.storage().instance().set(&DataKey::Spent, &0i128);
        env.storage()
            .instance()
            .set(&DataKey::Whitelist, &Map::<Address, bool>::new(&env));
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        let mut wl: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&DataKey::Whitelist)
            .unwrap();
        wl.set(recipient, true);
        env.storage().instance().set(&DataKey::Whitelist, &wl);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let wl: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&DataKey::Whitelist)
            .unwrap();
        if wl.get(to.clone()).unwrap_or(false) != true {
            return Err(Error::NotWhitelisted);
        }
        let cap: i128 = env.storage().instance().get(&DataKey::Cap).unwrap();
        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap();
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &to, &amount);
        env.storage().instance().set(&DataKey::Spent, &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).unwrap_or(0)
    }
}
