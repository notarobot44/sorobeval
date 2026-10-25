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
    fn to_bytes(&self, _env: &Env) -> Vec<u8> {
        vec![]
    }
    fn from_bytes(_env: &Env, _b: Vec<u8>) -> Result<Self, soroban_sdk::storage::TryFromValErr> {
        Ok(AdminKey)
    }
}

pub struct TokenKey;
impl soroban_sdk::storage::Storable for TokenKey {
    const MAX_SIZE: u32 = 100;
    fn to_bytes(&self, _env: &Env) -> Vec<u8> {
        vec![]
    }
    fn from_bytes(_env: &Env, _b: Vec<u8>) -> Result<Self, soroban_sdk::storage::TryFromValErr> {
        Ok(TokenKey)
    }
}

pub struct CapKey;
impl soroban_sdk::storage::Storable for CapKey {
    const MAX_SIZE: u32 = 100;
    fn to_bytes(&self, _env: &Env) -> Vec<u8> {
        vec![]
    }
    fn from_bytes(_env: &Env, _b: Vec<u8>) -> Result<Self, soroban_sdk::storage::TryFromValErr> {
        Ok(CapKey)
    }
}

pub struct SpentKey;
impl soroban_sdk::storage::Storable for SpentKey {
    const MAX_SIZE: u32 = 100;
    fn to_bytes(&self, _env: &Env) -> Vec<u8> {
        vec![]
    }
    fn from_bytes(_env: &Env, _b: Vec<u8>) -> Result<Self, soroban_sdk::storage::TryFromValErr> {
        Ok(SpentKey)
    }
}

pub struct WhitelistKey;
impl soroban_sdk::storage::Storable for WhitelistKey {
    const MAX_SIZE: u32 = 100;
    fn to_bytes(&self, _env: &Env) -> Vec<u8> {
        vec![]
    }
    fn from_bytes(_env: &Env, _b: Vec<u8>) -> Result<Self, soroban_sdk::storage::TryFromValErr> {
        Ok(WhitelistKey)
    }
}

pub struct InitializedKey;
impl soroban_sdk::storage::Storable for InitializedKey {
    const MAX_SIZE: u32 = 100;
    fn to_bytes(&self, _env: &Env) -> Vec<u8> {
        vec![]
    }
    fn from_bytes(_env: &Env, _b: Vec<u8>) -> Result<Self, soroban_sdk::storage::TryFromValErr> {
        Ok(InitializedKey)
    }
}

pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
    let initialized = env
        .storage()
        .instance()
        .get::<InitializedKey, bool>(&InitializedKey)
        .unwrap_or(false);
    if initialized {
        return Err(Error::AlreadyInitialized);
    }
    if cap <= 0 {
        return Err(Error::InvalidAmount);
    }

    env.storage().instance().set(&InitializedKey, &true);
    env.storage()
        .instance()
        .set::<AdminKey, Address>(&AdminKey, &admin);
    env.storage()
        .instance()
        .set::<TokenKey, Address>(&TokenKey, &token);
    env.storage().instance().set::<CapKey, i128>(&CapKey, &cap);
    let whitelist: soroban_sdk::vec::Vec<Address> = soroban_sdk::vec::Vec::new(env);
    env.storage()
        .instance()
        .set::<WhitelistKey, soroban_sdk::vec::Vec<Address>>(&WhitelistKey, &whitelist);
    env.storage().instance().set::<SpentKey, i128>(&SpentKey, &0);

    Ok(())
}

pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
    let admin = env
        .storage()
        .instance()
        .get::<AdminKey, Address>(&AdminKey)
        .ok_or(Error::NotInitialized)?;
    env.invoke_function(
        &admin,
        &soroban_sdk::symbol_short!("require_auth"),
        soroban_sdk::vec![&env.current_contract_address()],
    );

    let mut whitelist = env
        .storage()
        .instance()
        .get::<WhitelistKey, soroban_sdk::vec::Vec<Address>>(&WhitelistKey)
        .unwrap_or(soroban_sdk::vec::Vec::new(env));
    if !whitelist.contains(&recipient) {
        whitelist.push_back(recipient);
        env.storage()
            .instance()
            .set::<WhitelistKey, soroban_sdk::vec::Vec<Address>>(&WhitelistKey, &whitelist);
    }
    Ok(())
}

pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
    let admin = env
        .storage()
        .instance()
        .get::<AdminKey, Address>(&AdminKey)
        .ok_or(Error::NotInitialized)?;
    env.invoke_function(
        &admin,
        &soroban_sdk::symbol_short!("require_auth"),
        soroban_sdk::vec![&env.current_contract_address()],
    );

    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }

    let whitelist = env
        .storage()
        .instance()
        .get::<WhitelistKey, soroban_sdk::vec::Vec<Address>>(&WhitelistKey)
        .unwrap_or(soroban_sdk::vec::Vec::new(env));
    if !whitelist.contains(&to) {
        return Err(Error::NotWhitelisted);
    }

    let cap = env
        .storage()
        .instance()
        .get::<CapKey, i128>(&CapKey)
        .ok_or(Error::NotInitialized)?;
    let spent = env
        .storage()
        .instance()
        .get::<SpentKey, i128>(&SpentKey)
        .unwrap_or(0);
    if spent + amount > cap {
        return Err(Error::CapExceeded);
    }

    let token = env
        .storage()
        .instance()
        .get::<TokenKey, Address>(&TokenKey)
        .ok_or(Error::NotInitialized)?;
    let client = Client::new(&env, &token);
    client.transfer(
        &env.current_contract_address(),
        &to,
        &amount,
    );

    env.storage().instance().set::<SpentKey, i128>(&SpentKey, &(spent + amount));
    Ok(())
}

pub fn spent(env: Env) -> i128 {
    env.storage()
        .instance()
        .get::<SpentKey, i128>(&SpentKey)
        .unwrap_or(0)
}

pub fn cap(env: Env) -> i128 {
    env.storage()
        .instance()
        .get::<CapKey, i128>(&CapKey)
        .unwrap_or(0)
}
