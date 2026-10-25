#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env, IntoVal, Symbol, Vec,
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

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        // Check if already initialized
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        // Validate cap
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Store admin, token, and cap
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        env.storage().instance().set(&DataKey::Spent, &0i128);

        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        // Store the whitelisted recipient
        env.storage().persistent().set(&DataKey::Whitelist(recipient), &true);

        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        // Validate amount
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Check if recipient is whitelisted
        if !env.storage().persistent().has(&DataKey::Whitelist(to)) {
            return Err(Error::NotWhitelisted);
        }

        // Get current spent amount and cap
        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap_or(0);
        let cap: i128 = env.storage().instance().get(&DataKey::Cap).unwrap();

        // Check if spending would exceed the cap
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        // Get token address
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();

        // Transfer tokens from contract to recipient
        let token_client = soroban_sdk::token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &to, &amount);

        // Update spent amount
        env.storage().instance().set(&DataKey::Spent, &(spent + amount));

        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).unwrap()
    }
}
