#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

const COUNTER_KEY: Symbol = Symbol::short("c");

#[derive(Default)]
pub struct Counter;

#[contract]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let storage = env.storage().instance();
        let mut value: u32 = storage.get(&COUNTER_KEY).unwrap_or(0);
        value += 1;
        storage.set(&COUNTER_KEY, &value);
        // Extend the instance TTL to the default expiry
        storage.extend_expiration(&COUNTER_KEY, None);
        value
    }

    pub fn get(env: Env) -> u32 {
        let storage = env.storage().instance();
        storage.get(&COUNTER_KEY).unwrap_or(0)
    }
}
