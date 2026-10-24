#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, token, Address, Env,
};

const DAY_IN_LEDGERS: u32 = 17280;
const BUMP_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const BUMP_TO: u32 = 120 * DAY_IN_LEDGERS;

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
    Whitelisted(Address),
}

#[contractevent]
pub struct Whitelisted {
    #[topic]
    pub recipient: Address,
}

#[contractevent]
pub struct Spent {
    #[topic]
    pub to: Address,
    pub amount: i128,
    pub total_spent: i128,
}

#[contract]
pub struct Treasury;

fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(BUMP_THRESHOLD, BUMP_TO);
}

fn require_admin(env: &Env) -> Result<Address, Error> {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)?;
    admin.require_auth();
    Ok(admin)
}

fn is_whitelisted(env: &Env, who: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Whitelisted(who.clone()))
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
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        env.storage().instance().set(&DataKey::Spent, &0i128);
        extend_instance(&env);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        require_admin(&env)?;
        let key = DataKey::Whitelisted(recipient.clone());
        env.storage().persistent().set(&key, &true);
        env.storage()
            .persistent()
            .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TO);
        extend_instance(&env);
        Whitelisted { recipient }.publish(&env);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        require_admin(&env)?;

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !is_whitelisted(&env, &to) {
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

        // Overflow implies the sum exceeds i128::MAX >= cap, so it is a cap breach.
        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_spent > cap {
            return Err(Error::CapExceeded);
        }

        // Update state before the external call.
        env.storage().instance().set(&DataKey::Spent, &new_spent);
        extend_instance(&env);

        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &to, &amount);

        Spent {
            to,
            amount,
            total_spent: new_spent,
        }
        .publish(&env);
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
            .unwrap_or(0)
    }
}
