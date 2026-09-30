#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const COUNTER_KEY: soroban_sdk::Symbol = symbol_short!("COUNTER");
const INSTANCE_TTL: u32 = 100_000;
const LEDGERS_TO_EXTEND: u32 = 50_000;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Returns the current value of the counter without changing it.
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER_KEY).unwrap_or(0)
    }

    /// Increases the counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        let storage = env.storage().instance();
        let current: u32 = storage.get(&COUNTER_KEY).unwrap_or(0);
        let new_value = current + 1;
        storage.set(&COUNTER_KEY, &new_value);
        storage.extend_ttl(INSTANCE_TTL, LEDGERS_TO_EXTEND);
        new_value
    }
}
