#![no_std]

use soroban_sdk::{contract, contractimpl, Env, symbol_short};

/// Contract that keeps a simple persistent counter.
#[contract]
pub struct Counter;

const DAY_IN_LEDGERS: u32 = 17_280; // ~5 s per ledger

#[contractimpl]
impl Counter {
    /// Increment the counter by one and return the new value.
    pub fn increment(env: Env) -> u32 {
        let key = symbol_short!("ctr");
        // Read current value, defaulting to zero.
        let current: u32 = env.storage().instance().get(&key).unwrap_or(0);
        let next = current + 1;
        // Persist the new value.
        env.storage().instance().set(&key, &next);

        // Keep the instance storage alive.  Extend when below a threshold.
        env.storage()
            .instance()
            .extend_ttl(30 * DAY_IN_LEDGERS, 120 * DAY_IN_LEDGERS);

        next
    }

    /// Return the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        let key = symbol_short!("ctr");
        env.storage().instance().get(&key).unwrap_or(0)
    }
}
