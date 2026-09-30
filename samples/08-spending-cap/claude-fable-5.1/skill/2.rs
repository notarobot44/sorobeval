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

fn read_admin(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)
}

fn require_admin(env: &Env) -> Result<Address, Error> {
    let admin = read_admin(env)?;
    admin.require_auth();
    Ok(admin)
}

fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(BUMP_THRESHOLD, BUMP_TO);
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
        let store = env.storage().instance();
        store.set(&DataKey::Admin, &admin);
        store.set(&DataKey::Token, &token);
        store.set(&DataKey::Cap, &cap);
        store.set(&DataKey::Spent, &0i128);
        bump_instance(&env);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        require_admin(&env)?;
        let key = DataKey::Whitelisted(recipient.clone());
        env.storage().persistent().set(&key, &true);
        env.storage()
            .persistent()
            .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TO);
        bump_instance(&env);
        Whitelisted { recipient }.publish(&env);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        require_admin(&env)?;

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let wl_key = DataKey::Whitelisted(to.clone());
        let whitelisted: bool = env.storage().persistent().get(&wl_key).unwrap_or(false);
        if !whitelisted {
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

        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;

        // Update state before the external call.
        env.storage().instance().set(&DataKey::Spent, &new_spent);
        bump_instance(&env);
        env.storage()
            .persistent()
            .extend_ttl(&wl_key, BUMP_THRESHOLD, BUMP_TO);

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
        env.storage().instance().get(&DataKey::Cap).unwrap_or(0)
    }
}
