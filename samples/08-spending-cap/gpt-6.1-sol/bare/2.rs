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

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
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
        env.storage().instance().set(&DataKey::Spent, &0_i128);

        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let config = load_config(&env)?;
        config.admin.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Whitelisted(recipient), &true);

        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let config = load_config(&env)?;
        config.admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        if !env
            .storage()
            .instance()
            .has(&DataKey::Whitelisted(to.clone()))
        {
            return Err(Error::NotWhitelisted);
        }

        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap_or(0);
        let next_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;

        if next_spent > config.cap {
            return Err(Error::CapExceeded);
        }

        // Reserve the expenditure before the external call. A failed transfer
        // rolls back this write along with the rest of the invocation.
        env.storage()
            .instance()
            .set(&DataKey::Spent, &next_spent);

        token::Client::new(&env, &config.token).transfer(
            &env.current_contract_address(),
            &to,
            &amount,
        );

        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        match load_config(&env) {
            Ok(config) => config.cap,
            Err(error) => panic_with_error!(&env, error),
        }
    }
}
