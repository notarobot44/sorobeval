#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl,
    token::{Client as TokenClient},
    Address, Env,
};

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
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        env.storage().instance().set(&DataKey::Spent, &0i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();
        env.storage()
            .instance()
            .update(&DataKey::Whitelist(recipient.clone()), |whitelisted: Option<bool>| {
                Some(true)
            });
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !Self::is_whitelisted(&env, &to) {
            return Err(Error::NotWhitelisted);
        }
        let cap = env.storage().instance().get(&DataKey::Cap).unwrap();
        let spent = env.storage().instance().get(&DataKey::Spent).unwrap_or(0);
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let client = TokenClient::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &to, &amount);
        env.storage()
            .instance()
            .update(&DataKey::Spent, |s: Option<i128>| Some(s.unwrap_or(0) + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).unwrap()
    }

    fn require_admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    fn is_whitelisted(env: &Env, addr: &Address) -> bool {
        env.storage()
            .instance()
            .has(&DataKey::Whitelist(addr.clone()))
    }
}

#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelist(Address),
}
