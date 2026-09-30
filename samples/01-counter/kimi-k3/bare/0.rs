#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

const INSTANCE_LIFETIME_THRESHOLD: u32 = 50;
const INSTANCE_BUMP_AMOUNT: u32 = 100;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increases the persistent counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let new_count = count + 1;
        env.storage().instance().set(&COUNTER, &new_count);
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        new_count
    }

    /// Returns the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}
