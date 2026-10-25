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
    const NAME: &'static str = "Admin";
    fn to_type() -> soroban_sdk::storage::TypeMappings {
        soroban_sdk::storage::TypeMappings::Init(soroban_sdk::storage::SingleType::<Address>::new())
    }
}

pub struct TokenKey;
impl soroban_sdk::storage::Storable for TokenKey {
    const NAME: &'static str = "Token";
    fn to_type() -> soroban_sdk::storage::TypeMappings {
        soroban_sdk::storage::TypeMappings::Init(soroban_sdk::storage::SingleType::<Address>::new())
    }
}

pub struct CapKey;
impl soroban_sdk::storage::Storable for CapKey {
    const NAME: &'static str = "Cap";
    fn to_type() -> soroban_sdk::storage::TypeMappings {
        soroban_sdk::storage::TypeMappings::Init(soroban_sdk::storage::SingleType::<i128>::new())
    }
}

pub struct SpentKey;
impl soroban_sdk::storage::Storable for SpentKey {
    const NAME: &'static str = "Spent";
    fn to_type() -> soroban_sdk::storage::TypeMappings {
        soroban_sdk::storage::TypeMappings::Init(soroban_sdk::storage::SingleType::<i128>::new())
    }
}

pub struct WhitelistKey;
impl soroban_sdk::storage::Storable for WhitelistKey {
    const NAME: &'static str = "Whitelist";
    fn to_type() -> soroban_sdk::storage::TypeMappings {
        soroban_sdk::storage::TypeMappings::Init(soroban_sdk::storage::SingleType::<Vec<Address>>::new())
    }
}

pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
    if env.storage().instance().has(&AdminKey {}) {
        return Err(Error::AlreadyInitialized);
    }
    if cap <= 0 {
        return Err(Error::InvalidAmount);
    }

    let mut admin_storage = env.storage().instance().persistent().get::<AdminKey, Address>(&AdminKey {}).unwrap();
    *admin_storage = admin;
    let mut token_storage = env.storage().instance().persistent().get::<TokenKey, Address>(&TokenKey {}).unwrap();
    *token_storage = token;
    let mut cap_storage = env.storage().instance().persistent().get::<CapKey, i128>(&CapKey {}).unwrap();
    *cap_storage = cap;
    let mut spent_storage = env.storage().instance().persistent().get::<SpentKey, i128>(&SpentKey {}).unwrap();
    *spent_storage = 0;
    let mut whitelist_storage = env.storage().instance().persistent().get::<WhitelistKey, Vec<Address>>(&WhitelistKey {}).unwrap();
    *whitelist_storage = Vec::new();

    Ok(())
}

pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
    let admin = env.storage().instance().persistent().get::<AdminKey, Address>(&AdminKey {}).ok_or(Error::NotInitialized)?;
    env.invoke_contract(&admin, &soroban_sdk::contractimport::Address::from_address(&admin), &soroban_sdk::symbol_short!("require_auth"), vec![soroban_sdk::symbol_short!("require_auth")])?;
    let mut whitelist = env.storage().instance().persistent().get::<WhitelistKey, Vec<Address>>(&WhitelistKey {}).unwrap();
    if !whitelist.contains(&recipient) {
        whitelist.push(recipient);
    }
    Ok(())
}

pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
    let admin = env.storage().instance().persistent().get::<AdminKey, Address>(&AdminKey {}).ok_or(Error::NotInitialized)?;
    env.invoke_contract(&admin, &soroban_sdk::contractimport::Address::from_address(&admin), &soroban_sdk::symbol_short!("require_auth"), vec![soroban_sdk::symbol_short!("require_auth")])?;
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }
    let whitelist = env.storage().instance().persistent().get::<WhitelistKey, Vec<Address>>(&WhitelistKey {}).unwrap();
    if !whitelist.contains(&to) {
        return Err(Error::NotWhitelisted);
    }
    let cap = env.storage().instance().persistent().get::<CapKey, i128>(&CapKey {}).unwrap();
    let mut spent = env.storage().instance().persistent().get::<SpentKey, i128>(&SpentKey {}).unwrap();
    if spent + amount > cap {
        return Err(Error::CapExceeded);
    }
    spent += amount;
    let token = env.storage().instance().persistent().get::<TokenKey, Address>(&TokenKey {}).unwrap();
    Client::new(&env, &token).transfer(&env.current_contract_address(), &to, &amount)?;
    Ok(())
}

pub fn spent(env: Env) -> i128 {
    env.storage().instance().persistent().get::<SpentKey, i128>(&SpentKey {}).unwrap()
}

pub fn cap(env: Env) -> i128 {
    env.storage().instance().persistent().get::<CapKey, i128>(&CapKey {}).unwrap()
}
