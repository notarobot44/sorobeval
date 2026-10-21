#![no_std]

use soroban_sdk::{contractimpl, Env, Symbol};

pub struct Counter;

#[contractimpl]
impl Counter {
    const KEY: Symbol = Symbol::short("c");

    /// Increments the counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        let storage = env.storage().instance();
        // Read current value or default to 0
        let mut val: u32 = storage.get(&Self::KEY).unwrap_or(0);
        val += 1;
        // Store updated value
        storage.set(&Self::KEY, &val);
        // Extend the instance TTL (assumes a function `extend_ttl` exists)
        // This is a placeholder; replace with the actual SDK method if different.
        env.storage()
            .instance()
            .extend_ttl(soroban_sdk::Expiration::from_time(env.clock().time()));
        val
    }

    /// Returns the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        let storage = env.storage().instance();
        storage.get(&Self::KEY).unwrap_or(0)
    }
}
