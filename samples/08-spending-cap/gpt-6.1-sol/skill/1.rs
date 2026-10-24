#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token::Client,
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
    pub fn init(
        env: Env,
        admin: Address,
        token: Address,
        cap: i128,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        env.storage().instance().set(&DataKey::Spent, &0i128);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        require_admin(&env)?;

        env.storage()
            .persistent()
            .set(&DataKey::Whitelisted(recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        require_admin(&env)?;

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !env
            .storage()
            .persistent()
            .get::<_, bool>(&DataKey::Whitelisted(to.clone()))
            .unwrap_or(false)
        {
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
        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;

        if new_spent > cap {
            return Err(Error::CapExceeded);
        }

        let token: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;

        // A failed transfer rolls back this update atomically.
        env.storage().instance().set(&DataKey::Spent, &new_spent);
        Client::new(&env, &token).transfer(
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
