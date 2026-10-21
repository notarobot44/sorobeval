#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, token::Client as TokenClient,
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
    // init must be called once. It does not require auth because it is
    // typically invoked as the constructor during deployment.
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        // spent starts at 0
        env.storage().instance().set(&DataKey::Spent, &0i128);
        Ok(())
    }

    fn require_admin(env: &Env) -> Result<(), Error> {
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NotInitialized);
        }
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();
        admin.require_auth();
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        Self::require_admin(&env)?;
        let key = DataKey::Whitelist(recipient);
        env.storage().persistent().set(&key, &());
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        Self::require_admin(&env)?;

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        // check whitelist
        let wl_key = DataKey::Whitelist(to.clone());
        if !env.storage().persistent().has(&wl_key) {
            return Err(Error::NotWhitelisted);
        }

        // cap enforcement
        let cap: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Cap)
            .unwrap();
        let spent: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Spent)
            .unwrap_or(0i128);
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        // perform transfer from contract to recipient
        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .unwrap();
        let client = TokenClient::new(&env, &token_addr);
        let from = env.current_contract_address();
        client.transfer(&from, &to, &amount);

        // update spent
        env.storage().instance().set(&DataKey::Spent, &(spent + amount));
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Spent)
            .unwrap_or(0i128)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).unwrap_or(0i128)
    }
}
