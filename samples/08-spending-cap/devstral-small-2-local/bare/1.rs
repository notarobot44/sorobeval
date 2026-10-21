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

pub struct State {
    pub admin: Address,
    pub token: Address,
    pub cap: i128,
    pub spent: i128,
    pub whitelist: Vec<Address>,
    pub initialized: bool,
}

fn get_state(e: &Env) -> State {
    let mut state = e.storage().persistent().get::<State>(&"state").unwrap_or_default();
    if !state.initialized {
        return State {
            admin: Address::generate(e),
            token: Address::generate(e),
            cap: 0,
            spent: 0,
            whitelist: Vec::new(),
            initialized: false,
        };
    }
    state
}

fn set_state(e: &Env, state: &State) {
    e.storage().persistent().set(&"state", state);
}

pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
    let mut state = get_state(&env);
    if state.initialized {
        return Err(Error::AlreadyInitialized);
    }
    if cap <= 0 {
        return Err(Error::InvalidAmount);
    }

    state.admin = admin;
    state.token = token;
    state.cap = cap;
    state.spent = 0;
    state.whitelist = Vec::new();
    state.initialized = true;

    set_state(&env, &state);
    Ok(())
}

pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
    let mut state = get_state(&env);
    if !state.initialized {
        return Err(Error::NotInitialized);
    }
    env.storage().instance().require_auth(&state.admin);

    if !state.whitelist.contains(&recipient) {
        state.whitelist.push(recipient);
        set_state(&env, &state);
    }

    Ok(())
}

pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
    let mut state = get_state(&env);
    if !state.initialized {
        return Err(Error::NotInitialized);
    }
    env.storage().instance().require_auth(&state.admin);

    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }

    if !state.whitelist.contains(&to) {
        return Err(Error::NotWhitelisted);
    }

    let new_spent = state.spent + amount;
    if new_spent > state.cap {
        return Err(Error::CapExceeded);
    }

    state.spent = new_spent;
    set_state(&env, &state);

    let client = Client::new(&env, &state.token);
    client.transfer(
        &env.current_contract_address(),
        &to,
        &amount,
    );

    Ok(())
}

pub fn spent(env: Env) -> i128 {
    get_state(&env).spent
}

pub fn cap(env: Env) -> i128 {
    let state = get_state(&env);
    if !state.initialized {
        return 0;
    }
    state.cap
}
