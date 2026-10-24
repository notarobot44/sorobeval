#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token::Client, Address, Env};

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
enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelist(Address),
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        admin.require_auth();
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
        env.storage().instance().extend_ttl(10000, 10000);
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
            .instance()
            .set(&DataKey::Whitelist(recipient), &true);
        env.storage().instance().extend_ttl(10000, 10000);
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
        let whitelisted: bool = env
            .storage()
            .instance()
            .get(&DataKey::Whitelist(to.clone()))
            .unwrap_or(false);
        if !whitelisted {
            return Err(Error::NotWhitelisted);
        }
        let cap: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Cap)
            .ok_or(Error::NotInitialized)?;
        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap_or(0);
        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_spent > cap {
            return Err(Error::CapExceeded);
        }
        let token: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let client = Client::new(&env, &token);
        let self_addr = env.current_contract_address();
        client.transfer(&self_addr, &to, &amount);
        env.storage().instance().set(&DataKey::Spent, &new_spent);
        env.storage().instance().extend_ttl(10000, 10000);
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).unwrap_or_else(|| {
            soroban_sdk::panic_with_error!(&env, Error::NotInitialized)
        })
    }
}
