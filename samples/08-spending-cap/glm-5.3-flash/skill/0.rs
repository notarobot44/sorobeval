#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token::Client, Address,
    Env,
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
pub enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelist,
}

const DAY_IN_LEDGERS: u32 = 17280;
const BUMP_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const BUMP_TO: u32 = 120 * DAY_IN_LEDGERS;

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        let storage = env.storage().instance();
        if storage.has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::Cap, &cap);
        storage.set(&DataKey::Spent, &0i128);
        storage.set(&DataKey::Whitelist, &false);

        storage.extend_ttl(BUMP_THRESHOLD, BUMP_TO);
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        Self::require_admin(&env)?;

        let storage = env.storage().instance();
        let whitelist: Map = storage.get(&DataKey::Whitelist).unwrap_or_default();
        if !whitelist.contains(&recipient) {
            whitelist.push(recipient.clone());
            storage.set(&DataKey::Whitelist, &whitelist);
        }

        storage.extend_ttl(BUMP_THRESHOLD, BUMP_TO);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        Self::require_admin(&env)?;

        if amount <= 0 {
            panic_with_error!(&env, Error::InvalidAmount);
        }

        let storage = env.storage().instance();
        let whitelist: Map = storage.get(&DataKey::Whitelist).unwrap_or_default();
        if !whitelist.contains(&to) {
            panic_with_error!(&env, Error::NotWhitelisted);
        }

        let cap: i128 = storage.get(&DataKey::Cap).unwrap_or_default();
        let spent: i128 = storage.get(&DataKey::Spent).unwrap_or_default();
        if spent + amount > cap {
            panic_with_error!(&env, Error::CapExceeded);
        }

        let token_id: Address = storage.get(&DataKey::Token).unwrap_or_default();
        let client = Client::new(&env, &token_id);
        client.transfer(&env.current_contract_address(), &to, &amount);

        storage.set(&DataKey::Spent, &(spent + amount));
        storage.extend_ttl(BUMP_THRESHOLD, BUMP_TO);
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Spent)
            .unwrap_or_default()
    }

    pub fn cap(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Cap)
            .unwrap_or_default()
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
}

use soroban_sdk::Vec;
type Map = Vec<Address>;
