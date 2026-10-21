#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, token::Client as TokenClient,
    Address, Env, Symbol,
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

#[contract]
pub struct Treasury;

#[derive(Clone)]
enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
}

/// Helper to get the admin address, or error if not set.
fn get_admin(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)
}

impl Treasury {
    /// Initializes the treasury. Must be called once.
    pub fn init(
        env: Env,
        admin: Address,
        token: Address,
        cap: i128,
    ) -> Result<(), Error> {
        if env
            .storage()
            .instance()
            .has(&DataKey::Admin)
        {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        // start spent at 0
        env.storage().instance().set(&DataKey::Spent, &0i128);
        Ok(())
    }

    /// Adds a recipient to the whitelist. Only admin may call.
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        // Use persistent storage for the whitelist map
        let mut wl =
            env.storage()
                .persistent()
                .get::<Symbol, Map<_, _>>(&DataKey::Spent.into_val(&env))
                .unwrap_or_default(); // use a dummy key to store map

        // We store a map keyed by Address with unit value
        let map_key = Symbol::new(&env, "whitelist");
        let mut map =
            env.storage()
                .persistent()
                .get::<Symbol, Map<Address, ()>>(&map_key)
                .unwrap_or_default();
        map.set(&recipient, &());
        env.storage().persistent().set(&map_key, &map);
        Ok(())
    }

    /// Spend tokens to a whitelisted recipient.
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let admin = get_admin(&env)?;
        admin.require_auth();

        // Check whitelist
        let map_key = Symbol::new(&env, "whitelist");
        let wl_map =
            env.storage()
                .persistent()
                .get::<Symbol, Map<Address, ()>>(&map_key)
                .unwrap_or_default();
        if !wl_map.has(&to) {
            return Err(Error::NotWhitelisted);
        }

        // Check cap
        let cap: i128 = env.storage().instance().get(&DataKey::Cap).unwrap();
        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap();
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        // Transfer tokens
        let token_addr: Address =
            env.storage().instance().get(&DataKey::Token).unwrap();
        let client = TokenClient::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &to, &amount);

        // Update spent
        env.storage()
            .instance()
            .set(&DataKey::Spent, &(spent + amount));
        Ok(())
    }

    /// Return total spent so far.
    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    /// Return the cap set at init.
    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Cap).unwrap()
    }
}
