#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Counter;

const COUNTER_KEY: Symbol = Symbol::short("c");

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let storage = env.storage().instance();

        // Load current value or start at 0
        let current = match storage.get(&COUNTER_KEY) {
            Some(v) => v,
            None => 0u32,
        };

        let new_value = current + 1;

        // Store the updated counter
        storage.set(&COUNTER_KEY, &new_value);

        // Extend TTL (use default extension)
        storage.extend_ttl(&COUNTER_KEY, None);

        new_value
    }

    pub fn get(env: Env) -> u32 {
        let storage = env.storage().instance();

        match storage.get(&COUNTER_KEY) {
            Some(v) => v,
            None => 0u32,
        }
    }
}
