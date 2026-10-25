#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, TokenClient};

#[contracterror]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contracttype]
pub struct Treasury {
    pub admin: Address,
    pub token: Address,
    pub cap: i128,
    pub spent: i128,
    pub whitelist: std::collections::HashSet<Address>,
}

#[contract]
pub struct TreasuryContract;

#[contractimpl]
impl TreasuryContract {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        let storage = env.storage().instance();
        if storage.has::<Treasury>() {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        let treasury = Treasury {
            admin,
            token,
            cap,
            spent: 0,
            whitelist: std::collections::HashSet::new(),
        };
        storage.set(&treasury);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let mut treasury = env.storage().instance().get::<Treasury>()?;
        treasury.admin.require_auth();
        treasury.whitelist.insert(recipient);
        env.storage().instance().set(&treasury);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let mut treasury = env.storage().instance().get::<Treasury>()?;
        treasury.admin.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !treasury.whitelist.contains(&to) {
            return Err(Error::NotWhitelisted);
        }
        if treasury.spent + amount > treasury.cap {
            return Err(Error::CapExceeded);
        }
        let token_client = TokenClient::new(&env, &treasury.token);
        token_client.transfer(&env.current_contract_address(), &to, &amount);
        treasury.spent += amount;
        env.storage().instance().set(&treasury);
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        let treasury = env.storage().instance().get::<Treasury>().unwrap_or_else(|_| Treasury {
            admin: Address::random(&env),
            token: Address::random(&env),
            cap: 0,
            spent: 0,
            whitelist: std::collections::HashSet::new(),
        });
        treasury.spent
    }

    pub fn cap(env: Env) -> i128 {
        let treasury = env.storage().instance().get::<Treasury>().unwrap_or_else(|_| Treasury {
            admin: Address::random(&env),
            token: Address::random(&env),
            cap: 0,
            spent: 0,
            whitelist: std::collections::HashSet::new(),
        });
        treasury.cap
    }
}
