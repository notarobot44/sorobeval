#![no_std]

use soroban_sdk::{contracterror, contractimpl, token, Address, Env};

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

pub struct TreasuryData {
    pub admin: Address,
    pub token: Address,
    pub cap: i128,
    pub spent: i128,
    pub whitelisted: Vec<Address>,
}

fn get_data(e: &Env) -> TreasuryData {
    e.storage()
        .instance()
        .get::<TreasuryData>()
        .unwrap_or(TreasuryData {
            admin: Address::generate(e),
            token: Address::generate(e),
            cap: 0,
            spent: 0,
            whitelisted: Vec::new(),
        })
}

fn set_data(e: &Env, data: &TreasuryData) {
    e.storage().instance().set(&data);
}

pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
    let mut data = get_data(&env);
    if data.cap != 0 {
        return Err(Error::AlreadyInitialized);
    }
    if cap <= 0 {
        return Err(Error::InvalidAmount);
    }
    data.admin = admin;
    data.token = token;
    data.cap = cap;
    set_data(&env, &data);
    Ok(())
}

pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
    let mut data = get_data(&env);
    if data.cap == 0 {
        return Err(Error::NotInitialized);
    }
    env.invoke_function(
        &data.admin,
        &soroban_sdk::symbol!("require_auth"),
        soroban_sdk::Vec::new(),
    )?;
    if !data.whitelisted.contains(&recipient) {
        data.whitelisted.push_back(recipient);
    }
    set_data(&env, &data);
    Ok(())
}

pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
    let mut data = get_data(&env);
    if data.cap == 0 {
        return Err(Error::NotInitialized);
    }
    env.invoke_function(
        &data.admin,
        &soroban_sdk::symbol!("require_auth"),
        soroban_sdk::Vec::new(),
    )?;
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }
    if !data.whitelisted.contains(&to) {
        return Err(Error::NotWhitelisted);
    }
    let new_spent = data.spent + amount;
    if new_spent > data.cap {
        return Err(Error::CapExceeded);
    }
    data.spent = new_spent;
    token::Client::new(env, &data.token).transfer(
        &env.current_contract_address(),
        &to,
        &amount,
    )?;
    set_data(&env, &data);
    Ok(())
}

pub fn spent(env: Env) -> i128 {
    get_data(&env).spent
}

pub fn cap(env: Env) -> i128 {
    let data = get_data(&env);
    if data.cap == 0 {
        return Err(Error::NotInitialized)?;
    }
    data.cap
}
