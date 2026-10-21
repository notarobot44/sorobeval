#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Counter;

const COUNTER_KEY: Symbol = symbol_short!("cnt");

#[contractimpl]
impl Counter {
    /// Increments the persistent counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        // Read current value (default 0 if not set).
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(0);

        // Increment and store back.
        count += 1;
        env.storage().instance().set(&COUNTER_KEY, &count);

        // Extend the instance TTL so the counter stays warm.
        // Use a threshold of 30 days and extend to 120 days (in ledgers).
        const DAY_IN_LEDGERS: u32 = 17_280;
        const THRESHOLD: u32 = 30 * DAY_IN_LEDGERS; // 30 days
        const EXTEND_TO: u32 = 120 * DAY_IN_LEDGERS; // 120 days

        env.storage()
            .instance()
            .extend_ttl(THRESHOLD, EXTEND_TO);

        count
    }

    /// Returns the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER_KEY).unwrap_or(0)
    }
}
