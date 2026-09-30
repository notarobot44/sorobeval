#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token, Address, Env,
};

const TTL_THRESHOLD: u32 = 30 * 17_280;
const TTL_EXTEND_TO: u32 = 120 * 17_280;

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
    Config,
    Spent,
    Whitelisted(Address),
}

#[contracttype]
#[derive(Clone)]
struct Config {
    admin: Address,
    token: Address,
    cap: i128,
}

fn load_config(env: &Env) -> Result<Config, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Config)
        .ok_or(Error::NotInitialized)
}

fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(
        env: Env,
        admin: Address,
        token: Address,
        cap: i128,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Config) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        admin.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Config, &Config { admin, token, cap });
        env.storage().instance().set(&DataKey::Spent, &0i128);
        extend_instance_ttl(&env);

        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let config = load_config(&env)?;
        config.admin.require_auth();

        let key = DataKey::Whitelisted(recipient);
        env.storage().persistent().set(&key, &true);
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
        extend_instance_ttl(&env);

        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let config = load_config(&env)?;
        config.admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let key = DataKey::Whitelisted(to.clone());
        let allowed: bool = env.storage().persistent().get(&key).unwrap_or(false);
        if !allowed {
            return Err(Error::NotWhitelisted);
        }

        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap_or(0);
        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_spent > config.cap {
            return Err(Error::CapExceeded);
        }

        // A failed token transfer atomically rolls back this update.
        env.storage().instance().set(&DataKey::Spent, &new_spent);
        extend_instance_ttl(&env);
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);

        let destination = to.into();
        token::Client::new(&env, &config.token).transfer(
            &env.current_contract_address(),
            &destination,
            &amount,
        );

        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        let config: Option<Config> = env.storage().instance().get(&DataKey::Config);
        config.map(|config| config.cap).unwrap_or(0)
    }
}
