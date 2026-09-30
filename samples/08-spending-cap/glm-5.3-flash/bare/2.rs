#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token::Client, Address, Env,
};

#[contracterror]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// One-time initialization: stores admin, token address and the
    /// immutable lifetime spending cap.
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        let storage = env.storage().instance();

        if storage.has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::Cap, &cap);
        storage.set(&DataKey::Spent, &0i128);

        Ok(())
    }

    /// Admin-only: add a recipient to the whitelist.
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;

        admin.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Whitelisted(recipient), &true);

        Ok(())
    }

    /// Admin-only: pay a whitelisted recipient out of the contract's own
    /// token balance, enforcing the immutable lifetime cap.
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let storage = env.storage().instance();

        let admin: Address = storage.get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        let token: Address = storage.get(&DataKey::Token).ok_or(Error::NotInitialized)?;
        let cap: i128 = storage.get(&DataKey::Cap).ok_or(Error::NotInitialized)?;
        let spent: i128 = storage.get(&DataKey::Spent).unwrap_or(0);

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !env
            .storage()
            .persistent()
            .has(&DataKey::Whitelisted(to.clone()))
        {
            return Err(Error::NotWhitelisted);
        }

        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_spent > cap {
            return Err(Error::CapExceeded);
        }

        admin.require_auth();

        let client = Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &to, &amount);

        storage.set(&DataKey::Spent, &new_spent);

        Ok(())
    }

    /// Running total of tokens paid out by the treasury.
    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    /// The immutable lifetime cap set at init.
    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).unwrap_or(0)
    }
}
