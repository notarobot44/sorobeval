#![no_std]

use soroban_sdk::{
    contract, contractimpl,
    symbol_short, Env, Symbol,
};

const DAY_IN_LEDGERS: u32 = 17_280;
const TTL_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;           // when to bump
const TTL_EXTEND_TO: u32 = 180 * DAY_IN_LEDGERS;          // target TTL

/// Key used in instance storage for the counter.
const COUNTER_KEY: Symbol = symbol_short!("counter");

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increment the persistent counter by one and return the new value.
    pub fn increment(env: Env) -> u32 {
        // Read current value, defaulting to 0.
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(0);

        // Increment and store back.
        count = count + 1;
        env.storage().instance().set(&COUNTER_KEY, &count);

        // Extend the instance TTL so the counter is not archived.
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        count
    }

    /// Return the current value of the counter without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER_KEY).unwrap_or(0)
    }
}
