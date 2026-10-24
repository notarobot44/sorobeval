#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token::Client, Address, Env};

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

#[contracttype]
#[derive(Clone)]
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
        Ok(())
    }

    fn require_admin(env: &Env) -> Result<Address, Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        Ok(admin)
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let _admin = Self::require_admin(&env)?;
        let mut list = Self::whitelisted(&env);
        if !list.contains(&recipient) {
            list.push_back(recipient);
            env.storage().instance().set(&DataKey::Whitelist, &list);
        }
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let _admin = Self::require_admin(&env)?;
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !Self::whitelisted(&env).contains(&to) {
            return Err(Error::NotWhitelisted);
        }
        let cap: i128 = env.storage().instance().get(&DataKey::Cap).unwrap_or(0);
        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap_or(0);
        if spent.checked_add(amount).ok_or(Error::CapExceeded)? > cap {
            return Err(Error::CapExceeded);
        }
        let token: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let client = Client::new(&env, &token);
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

    fn whitelisted(env: &Env) -> soroban_sdk::Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Whitelist)
            .unwrap_or(soroban_sdk::Vec::new(env))
    }
}
