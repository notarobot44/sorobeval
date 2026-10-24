#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};
use soroban_sdk::token::Client;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelisted(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        env.storage().instance().set(&DataKey::Spent, &0i128);
        env.storage().instance().extend_ttl(17280, 3_110_400);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::Whitelisted(recipient.clone()), &true);
        env.storage().instance().extend_ttl(17280, 3_110_400);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        let token: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let cap: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Cap)
            .ok_or(Error::NotInitialized)?;
        let spent: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Spent)
            .unwrap_or(0);
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let is_whitelisted: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Whitelisted(to.clone()))
            .unwrap_or(false);
        if !is_whitelisted {
            return Err(Error::NotWhitelisted);
        }
        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_spent > cap {
            return Err(Error::CapExceeded);
        }
        let self_addr = env.current_contract_address();
        Client::new(&env, &token).transfer(&self_addr, &to, &amount);
        env.storage().instance().set(&DataKey::Spent, &new_spent);
        env.storage().instance().extend_ttl(17280, 3_110_400);
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).unwrap_or(0)
    }
}
