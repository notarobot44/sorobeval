#![no_std]
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env, IntoVal, Vec,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelist(Address),
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

#[contractevent]
pub struct Spent {
    #[topic]
    pub to: Address,
    pub amount: i128,
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
        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();
        
        env.storage().persistent().set(&DataKey::Whitelist(recipient), &true);
        Ok(())
    }
    
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();
        
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        let token: Address = env.storage().instance().get(&DataKey::Token).ok_or(Error::NotInitialized)?;
        let cap: i128 = env.storage().instance().get(&DataKey::Cap).ok_or(Error::NotInitialized)?;
        
        if !env.storage().persistent().has(&DataKey::Whitelist(to)) {
            return Err(Error::NotWhitelisted);
        }
        
        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap_or(0);
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }
        
        let token_client = soroban_sdk::token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &to, &amount);
        
        env.storage().instance().set(&DataKey::Spent, &(spent + amount));
        Spent { to, amount }.publish(&env);
        Ok(())
    }
    
    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }
    
    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).ok_or(Error::NotInitialized).unwrap()
    }
}
