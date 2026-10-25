#![no_std]

use soroban_sdk::{
    contract,
    contracterror,
    contractimpl,
    token::Client as TokenClient,
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

#[derive(Clone, Copy, Eq, PartialEq)]
enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    /// Initializes the treasury. Can only be called once.
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        // Ensure not already initialized
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::Token, &token);
        env.storage()
            .instance()
            .set(&DataKey::Cap, &cap);
        // Spent starts at 0
        env.storage().instance().set(&DataKey::Spent, &0i128);

        Ok(())
    }

    /// Adds a recipient to the whitelist. Only callable by admin.
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        // Store an empty value as a marker for whitelisted addresses.
        env.storage()
            .persistent()
            .set(&recipient, &());
        Ok(())
    }

    /// Spends tokens from the treasury to a whitelisted recipient.
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Ensure recipient is whitelisted.
        if !env.storage().persistent().has(&to) {
            return Err(Error::NotWhitelisted);
        }

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

        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        // Transfer tokens from the contract itself to the recipient.
        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let token_client = TokenClient::new(&env, &token_addr);
        let contract_addr = env.current_contract_address();
        token_client.transfer(&contract_addr, &to, &amount);

        // Update spent total.
        env.storage()
            .instance()
            .set(&DataKey::Spent, &(spent + amount));

        Ok(())
    }

    /// Returns the cumulative amount already spent.
    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    /// Returns the hard cap set at initialization.
    pub fn cap(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Cap)
            .unwrap_or(0)
    }
}
