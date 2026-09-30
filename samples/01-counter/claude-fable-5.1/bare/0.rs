#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

const INSTANCE_LIFETIME_THRESHOLD: u32 = 100;
const INSTANCE_BUMP_AMOUNT: u32 = 100;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increment the counter by one and return the new value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        count
    }

    /// Return the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}
