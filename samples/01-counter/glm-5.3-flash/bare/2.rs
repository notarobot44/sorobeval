#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const COUNTER_KEY: soroban_sdk::Symbol = symbol_short!("COUNTER");
const INSTANCE_TTL: u32 = 100_000;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increases the counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(0);

        count = count.checked_add(1).unwrap_or_else(|| {
            panic!("counter overflow");
        });

        env.storage().instance().set(&COUNTER_KEY, &count);

        // Extend the instance storage TTL so the counter stays available.
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_TTL, INSTANCE_TTL);

        count
    }

    /// Returns the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(0)
    }
}
