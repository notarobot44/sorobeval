#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token::Client, Address,
    Env, Map,
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
        storage.set(&DataKey::Whitelist, &Map::<Address, bool>::new(&env));
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let storage = env.storage().instance();
        let admin: Address = storage.get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        let mut list: Map<Address, bool> = storage
            .get(&DataKey::Whitelist)
            .ok_or(Error::NotInitialized)?;
        list.set(recipient, true);
        storage.set(&DataKey::Whitelist, &list);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let storage = env.storage().instance();
        let admin: Address = storage.get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        let token: Address = storage.get(&DataKey::Token).ok_or(Error::NotInitialized)?;
        let cap: i128 = storage.get(&DataKey::Cap).ok_or(Error::NotInitialized)?;

        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let list: Map<Address, bool> = storage
            .get(&DataKey::Whitelist)
            .ok_or(Error::NotInitialized)?;
        if !list.get(to.clone()).unwrap_or(false) {
            panic_with_error!(&env, Error::NotWhitelisted);
        }

        let spent: i128 = storage.get(&DataKey::Spent).unwrap_or(0);
        if spent.checked_add(amount).unwrap_or(i128::MAX) > cap
            || spent.checked_add(amount).is_none()
        {
            panic_with_error!(&env, Error::CapExceeded);
        }

        let client = Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &to, &amount);

        storage.set(&DataKey::Spent, &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).unwrap_or(0)
    }
}
