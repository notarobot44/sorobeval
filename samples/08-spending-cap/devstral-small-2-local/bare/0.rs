#![no_std]

use soroban_sdk::{contracterror, contractimpl, token::Client, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contractimpl]
pub struct Treasury;

pub struct TreasuryDataKey;
impl soroban_sdk::storage::StorageKey for TreasuryDataKey {
    fn domain() -> &'static str {
        "Treasury"
    }
}

pub struct AdminKey;
impl soroban_sdk::storage::Storable for AdminKey {
    const MAX_SIZE: u32 = 100;
}
impl soroban_sdk::storage::StorageKey for AdminKey {
    fn domain() -> &'static str {
        "Admin"
    }
}

pub struct TokenKey;
impl soroban_sdk::storage::Storable for TokenKey {
    const MAX_SIZE: u32 = 100;
}
impl soroban_sdk::storage::StorageKey for TokenKey {
    fn domain() -> &'static str {
        "Token"
    }
}

pub struct CapKey;
impl soroban_sdk::storage::Storable for CapKey {
    const MAX_SIZE: u32 = 100;
}
impl soroban_sdk::storage::StorageKey for CapKey {
    fn domain() -> &'static str {
        "Cap"
    }
}

pub struct SpentKey;
impl soroban_sdk::storage::Storable for SpentKey {
    const MAX_SIZE: u32 = 100;
}
impl soroban_sdk::storage::StorageKey for SpentKey {
    fn domain() -> &'static str {
        "Spent"
    }
}

pub struct WhitelistKey;
impl soroban_sdk::storage::Storable for WhitelistKey {
    const MAX_SIZE: u32 = 100;
}
impl soroban_sdk::storage::StorageKey for WhitelistKey {
    fn domain() -> &'static str {
        "Whitelist"
    }
}

pub struct TreasuryData {
    pub admin: Address,
    pub token: Address,
    pub cap: i128,
    pub spent: i128,
    pub whitelist: Vec<Address>,
}

fn get_data(env: &Env) -> Result<TreasuryData, Error> {
    let key = TreasuryDataKey {};
    env.storage()
        .persistent()
        .get::<TreasuryDataKey, TreasuryData>(&key)
        .ok_or(Error::NotInitialized)
}

fn set_data(env: &Env, data: &TreasuryData) -> Result<(), Error> {
    let key = TreasuryDataKey {};
    env.storage().persistent().set(&key, data);
    Ok(())
}

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if get_data(&env).is_ok() {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        let data = TreasuryData {
            admin,
            token,
            cap,
            spent: 0,
            whitelist: Vec::new(),
        };
        set_data(&env, &data)?;
        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let mut data = get_data(&env)?;
        env.invoke_function(
            &data.admin,
            &soroban_sdk::symbol!("require_auth"),
            soroban_sdk::Symbol::new(&env, "require_auth"),
        )?;
        if !data.whitelist.contains(&recipient) {
            data.whitelist.push(recipient);
            set_data(&env, &data)?;
        }
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let mut data = get_data(&env)?;
        env.invoke_function(
            &data.admin,
            &soroban_sdk::symbol!("require_auth"),
            soroban_sdk::Symbol::new(&env, "require_auth"),
        )?;
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !data.whitelist.contains(&to) {
            return Err(Error::NotWhitelisted);
        }
        if data.spent + amount > data.cap {
            return Err(Error::CapExceeded);
        }

        let client = Client::new(&env, &data.token);
        client.transfer(
            &env.current_contract_address(),
            &to,
            &amount,
        )?;
        data.spent += amount;
        set_data(&env, &data)?;
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        get_data(&env).map(|d| d.spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        get_data(&env).map(|d| d.cap).unwrap_or(0)
    }
}
