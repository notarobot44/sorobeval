#![no_std]

use soroban_sdk::{
    contract, contractimpl, Env,
};

/// Short symbol key for the counter value stored in instance storage.
const COUNTER_KEY: &str = "cnt";

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increments the persistent counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        // Read current count, defaulting to 0 if not set yet.
        let mut count = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, COUNTER_KEY))
            .unwrap_or(0u32);

        // Increment and store back.
        count += 1;
        env.storage().instance().set(&Symbol::new(&env, COUNTER_KEY), &count);

        // Extend the instance TTL so the counter stays alive.
        const DAY_IN_LEDGERS: u32 = 17_280;          // ~5s per ledger
        const BUMP_THRESHOLD: u32 = 120 * DAY_IN_LEDGERS;
        const BUMP_TO: u32 = 180 * DAY_IN_LEDGERS;
        env.storage()
            .instance()
            .extend_ttl(BUMP_THRESHOLD, BUMP_TO);

        count
    }

    /// Returns the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, COUNTER_KEY))
            .unwrap_or(0u32)
    }
}
