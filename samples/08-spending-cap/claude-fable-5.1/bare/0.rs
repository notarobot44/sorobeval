#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
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
    Whitelisted(Address),
}

#[contract]
pub struct Treasury;

impl Treasury {
    fn load_admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    fn load_token(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Token)
            .ok_or(Error::NotInitialized)
    }

    fn load_cap(env: &Env) -> Result<i128, Error> {
        env.storage()
            .instance()
            .get::<DataKey, i128>(&DataKey::Cap)
            .ok_or(Error::NotInitialized)
    }

    fn load_spent(env: &Env) -> i128 {
        env.storage()
            .instance()
            .get::<DataKey, i128>(&DataKey::Spent)
            .unwrap_or(0)
    }

    fn is_whitelisted(env: &Env, who: &Address) -> bool {
        env.storage()
            .persistent()
            .get::<DataKey, bool>(&DataKey::Whitelisted(who.clone()))
            .unwrap_or(false)
    }
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
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = Self::load_admin(&env)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::Whitelisted(recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin = Self::load_admin(&env)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !Self::is_whitelisted(&env, &to) {
            return Err(Error::NotWhitelisted);
        }

        let cap = Self::load_cap(&env)?;
        let spent = Self::load_spent(&env);
        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_spent > cap {
            return Err(Error::CapExceeded);
        }

        // Update state before external call.
        env.storage().instance().set(&DataKey::Spent, &new_spent);

        let token_addr = Self::load_token(&env)?;
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &to, &amount);

        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        Self::load_spent(&env)
    }

    pub fn cap(env: Env) -> i128 {
        Self::load_cap(&env).unwrap_or(0)
    }
}
