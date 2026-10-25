#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token::Client as TokenClient,
    Address, Env,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelist(Address),
}

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

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
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
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        // spent starts at 0 implicitly
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        // mark as whitelisted; value is irrelevant
        env.storage()
            .instance()
            .set(&DataKey::Whitelist(recipient), &());
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let admin = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        // check whitelist
        if !env.storage().instance().has(&DataKey::Whitelist(to)) {
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

        // perform transfer from contract to recipient
        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let token_client = TokenClient::new(&env, &token_addr);

        let contract_addr = env.current_contract_address();
        token_client.transfer(&contract_addr, &to, &amount);

        // update spent total
        env.storage()
            .instance()
            .set(&DataKey::Spent, &(spent + amount));

        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Cap)
            .unwrap_or(0)
    }
}
