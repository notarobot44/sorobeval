#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const KEY: soroban_sdk::Symbol = symbol_short!("COUNTER");

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&KEY).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&KEY, &count);
        env.storage().instance().extend_ttl(50, 100);
        count
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&KEY).unwrap_or(0)
    }
}
