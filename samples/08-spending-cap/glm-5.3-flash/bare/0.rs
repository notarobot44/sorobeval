#![no_std]

use soroban_sdk::{contract, contracterror, contracttype, Address, Env, Map};
use soroban_sdk::token::Client as TokenClient;

#[contracterror]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelist,
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        let mut storage = env.storage().persistent();
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::Cap, &cap);
        storage.set(&DataKey::Spent, &0i128);
        storage.set(&DataKey::Whitelist, &Map::<Address, bool>::new(&env));
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        let mut map: Map<Address, bool> = env
            .storage()
            .persistent()
            .get(&DataKey::Whitelist)
            .ok_or(Error::NotInitialized)?;
        map.set(recipient, true);
        env.storage().persistent().set(&DataKey::Whitelist, &map);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let storage = env.storage().persistent();
        let admin: Address = storage.get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let whitelist: Map<Address, bool> = storage
            .get(&DataKey::Whitelist)
            .ok_or(Error::NotInitialized)?;
        if !whitelist.contains_key(&to) {
            return Err(Error::NotWhitelisted);
        }
        let cap: i128 = storage.get(&DataKey::Cap).ok_or(Error::NotInitialized)?;
        let spent: i128 = storage.get(&DataKey::Spent).unwrap_or(0);
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        let token = TokenClient::new(
            &env,
            &storage.get(&DataKey::Token).ok_or(Error::NotInitialized)?,
        );
        token.transfer(&env.current_contract_address(), &to, &amount);
        storage.set(&DataKey::Spent, &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::Cap).unwrap_or(0)
    }
}
