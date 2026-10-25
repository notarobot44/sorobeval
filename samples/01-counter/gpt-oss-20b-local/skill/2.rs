#![no_std]

use soroban_sdk::{
    contract, contractimpl,
    env::Env,
    symbol_short,
};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increments the persistent counter stored in instance storage.
    ///
    /// The counter is identified by a short symbol key.  On each call we
    /// read the current value (defaulting to `0`), increment it, write it back,
    /// and extend the instance TTL so the entry never expires during normal use.
    pub fn increment(env: Env) -> u32 {
        let key = symbol_short!("counter");
        // Load current value or start at zero.
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(0u32);
        // Increment and persist.
        count += 1;
        env.storage().instance().set(&key, &count);

        // Extend the instance TTL to keep the counter alive.  We use a very
        // large `extend_to` so that even if the host has shortened the TTL,
        // it will be reset on every increment.
        const EXTEND_THRESHOLD: u32 = 0;
        const EXTEND_TO: u32 = 180 * 17280; // ~180 days in ledgers
        env.storage()
            .instance()
            .extend_ttl(EXTEND_THRESHOLD, EXTEND_TO);

        count
    }

    /// Returns the current counter value without modifying state.
    pub fn get(env: Env) -> u32 {
        let key = symbol_short!("counter");
        env.storage().instance().get(&key).unwrap_or(0u32)
    }
}
