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
pub enum Key {
    Admin,
    Token,
    Cap,
    Spent,
    Allowed(Address),
}

#[contract]
pub struct Treasury;

fn admin(env: &Env) -> Result<Address, Error> {
    env.storage().instance().get(&Key::Admin).ok_or(Error::NotInitialized)
}

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&Key::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        let s = env.storage().instance();
        s.set(&Key::Admin, &admin);
        s.set(&Key::Token, &token);
        s.set(&Key::Cap, &cap);
        s.set(&Key::Spent, &0i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        admin(&env)?.require_auth();
        env.storage().persistent().set(&Key::Allowed(recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        admin(&env)?.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !env.storage().persistent().has(&Key::Allowed(to.clone())) {
            return Err(Error::NotWhitelisted);
        }
        let cap: i128 = env.storage().instance().get(&Key::Cap).ok_or(Error::NotInitialized)?;
        let spent: i128 = env.storage().instance().get(&Key::Spent).unwrap_or(0);
        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_spent > cap {
            return Err(Error::CapExceeded);
        }
        let token: Address = env.storage().instance().get(&Key::Token).ok_or(Error::NotInitialized)?;
        env.storage().instance().set(&Key::Spent, &new_spent);
        token::Client::new(&env, &token).transfer(&env.current_contract_address(), &to, &amount);
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&Key::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&Key::Cap).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
