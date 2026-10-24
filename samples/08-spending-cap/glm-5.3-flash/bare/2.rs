#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

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
    Whitelisted(Address),
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().persistent().get(&DataKey::Admin).is_some() {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        let mut s = env.storage().persistent();
        s.set(&DataKey::Admin, &admin);
        s.set(&DataKey::Token, &token);
        s.set(&DataKey::Cap, &cap);
        s.set(&DataKey::Spent, &0i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::Whitelisted(recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let s = env.storage().persistent();
        let admin: Address = s.get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !s.get(&DataKey::Whitelisted(to.clone())).unwrap_or(false) {
            return Err(Error::NotWhitelisted);
        }
        let cap: i128 = s.get(&DataKey::Cap).ok_or(Error::NotInitialized)?;
        let spent: i128 = s.get(&DataKey::Spent).unwrap_or(0);
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        let token: Address = s.get(&DataKey::Token).ok_or(Error::NotInitialized)?;
        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &to, &amount);

        s.set(&DataKey::Spent, &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::Cap).unwrap_or(0)
    }
}
