#![no_std]

use soroban_sdk::{
    contract, contractimpl,
    symbol_short, Env,
};

/// Symbol key used to store the counter in instance storage.
const COUNTER_KEY: Symbol = symbol_short!("counter");

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increments the persistent counter by one and returns the new value.
    ///
    /// The counter is stored in the contract's instance storage under the
    /// `COUNTER_KEY`.  The entry has a TTL that is extended on every call to keep
    /// it warm.  If the key does not exist yet, it starts at zero.
    pub fn increment(env: Env) -> u32 {
        // Read current value or start from 0.
        let mut count = env
            .storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(0u32);

        // Increment and write back.
        count += 1;
        env.storage().instance().set(&COUNTER_KEY, &count);

        // Extend the instance TTL to keep the counter from being archived.
        // Use a threshold of ~30 days (17280 ledgers) and extend to ~120 days.
        const THRESHOLD: u32 = 30 * 17_280;
        const EXTEND_TO: u32 = 120 * 17_280;
        env.storage()
            .instance()
            .extend_ttl(THRESHOLD, EXTEND_TO);

        count
    }

    /// Returns the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(0u32)
    }
}
