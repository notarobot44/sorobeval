#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const COUNTER_KEY: soroban_sdk::Symbol = symbol_short!("COUNTER");
const INSTANCE_TTL: u32 = 100_000;
const TTL_BUMP: u32 = 100;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Returns the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER_KEY).unwrap_or(0)
    }

    /// Increments the counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        let storage = env.storage().instance();
        let current: u32 = storage.get(&COUNTER_KEY).unwrap_or(0);
        let next = current + 1;
        storage.set(&COUNTER_KEY, &next);
        storage.extend_ttl(TTL_BUMP, INSTANCE_TTL);
        next
    }
}
