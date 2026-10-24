#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const COUNTER_KEY: soroban_sdk::Symbol = symbol_short!("counter");
const TTL: u32 = 100;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let mut val = env.storage().instance().get(&COUNTER_KEY).unwrap_or(0);
        val += 1;
        env.storage().instance().set(&COUNTER_KEY, &val);
        env.storage().instance().extend_ttl(TTL, TTL);
        val
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER_KEY).unwrap_or(0)
    }
}
