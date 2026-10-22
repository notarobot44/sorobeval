#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address, Env,
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
enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelisted(Address),
}

#[contract]
pub struct Treasury;

fn require_admin(env: &Env) -> Result<(), Error> {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)?;
    admin.require_auth();
    Ok(())
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

        admin.require_auth();

        // Keep all state in the same instance-storage lifetime.
        let storage = env.storage().instance();
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::Cap, &cap);
        storage.set(&DataKey::Spent, &0_i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        require_admin(&env)?;
        env.storage()
            .instance()
            .set(&DataKey::Whitelisted(recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        require_admin(&env)?;

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let storage = env.storage().instance();
        if !storage
            .get::<_, bool>(&DataKey::Whitelisted(to.clone()))
            .unwrap_or(false)
        {
            return Err(Error::NotWhitelisted);
        }

        let cap: i128 = storage
            .get(&DataKey::Cap)
            .ok_or(Error::NotInitialized)?;
        let spent: i128 = storage
            .get(&DataKey::Spent)
            .ok_or(Error::NotInitialized)?;
        let next_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;

        if next_spent > cap {
            return Err(Error::CapExceeded);
        }

        let token_address: Address = storage
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;

        // Reserve the outflow before the external call. A failed transfer
        // aborts the invocation and rolls back this update.
        storage.set(&DataKey::Spent, &next_spent);

        token::Client::new(&env, &token_address).transfer(
            &env.current_contract_address(),
            &to,
            &amount,
        );

        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Spent)
            .unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Cap)
            .unwrap_or_else(|| panic_with_error!(&env, Error::NotInitialized))
    }
}
