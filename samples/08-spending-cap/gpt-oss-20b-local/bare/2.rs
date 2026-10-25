#![no_std]

use soroban_sdk::{
    contractimpl, contracterror, Address, Env, IntoVal, Symbol,
    token::{self, Client as TokenClient},
};

#[contracterror]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

struct Treasury;

impl Treasury {
    const ADMIN_KEY: Symbol = Symbol::short("ADMIN");
    const TOKEN_KEY: Symbol = Symbol::short("TOKEN");
    const CAP_KEY: Symbol = Symbol::short("CAP");
    const SPENT_KEY: Symbol = Symbol::short("SPENT");

    fn require_admin(env: &Env) -> Result<(), Error> {
        let admin: Address = env.storage().get(&Self::ADMIN_KEY).unwrap();
        admin.require_auth();
        Ok(())
    }

    #[contractimpl]
    pub fn init(
        env: Env,
        admin: Address,
        token: Address,
        cap: i128,
    ) -> Result<(), Error> {
        if env.storage().has::<Address>(&Self::ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        env.storage()
            .set(&Self::ADMIN_KEY, &admin.into_val(&env));
        env.storage()
            .set(&Self::TOKEN_KEY, &token.into_val(&env));
        env.storage().set(&Self::CAP_KEY, &cap.into_val(&env));
        env.storage().set(&Self::SPENT_KEY, &0i128.into_val(&env));
        Ok(())
    }

    #[contractimpl]
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        Self::require_admin(&env)?;
        let key = Symbol::short("WL");
        let mut wl_set: Vec<Address> =
            env.storage().get(&key).unwrap_or_else(|| Vec::new(&env));
        if !wl_set.contains(&recipient) {
            wl_set.push_back(recipient);
            env.storage().set(&key, &wl_set.into_val(&env));
        }
        Ok(())
    }

    #[contractimpl]
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        Self::require_admin(&env)?;
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let key = Symbol::short("WL");
        let wl_set: Vec<Address> =
            env.storage().get(&key).unwrap_or_else(|| Vec::new(&env));
        if !wl_set.contains(&to) {
            return Err(Error::NotWhitelisted);
        }
        let cap_val: i128 = env
            .storage()
            .get(&Self::CAP_KEY)
            .expect("cap not initialized");
        let spent_val: i128 = env
            .storage()
            .get(&Self::SPENT_KEY)
            .unwrap_or_else(|| 0i128.into_val(&env));
        if spent_val + amount > cap_val {
            return Err(Error::CapExceeded);
        }
        let token_addr: Address = env.storage().get(&Self::TOKEN_KEY).unwrap();
        let token_client = TokenClient::new(&env, &token_addr);
        let from = env.current_contract_address();
        token_client.transfer(&from, &to, amount)?;
        env.storage()
            .set(&Self::SPENT_KEY, &(spent_val + amount).into_val(&env));
        Ok(())
    }

    #[contractimpl]
    pub fn spent(env: Env) -> i128 {
        env.storage()
            .get(&Self::SPENT_KEY)
            .unwrap_or_else(|| 0i128.into_val(&env))
    }

    #[contractimpl]
    pub fn cap(env: Env) -> i128 {
        env.storage().get(&Self::CAP_KEY).unwrap_or_else(|| 0i128.into_val(&env))
    }
}
