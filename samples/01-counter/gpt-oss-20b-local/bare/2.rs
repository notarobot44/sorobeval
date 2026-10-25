#![no_std]

use soroban_sdk::{
    contract::{contractimpl, Contract},
    Env,
    Symbol,
};

pub struct Counter;

const COUNTER_KEY: Symbol = Symbol::short("c");

#[contract]
impl Counter {
    /// Increment the persistent counter by one and return the new value.
    #[allow(clippy::unused_self)]
    pub fn increment(env: Env) -> u32 {
        // Retrieve current counter value, defaulting to 0 if not set
        let mut counter = env.storage().instance().get_or_default::<Symbol, u32>(&COUNTER_KEY);
        // Increment and store the new value
        counter += 1;
        env.storage()
            .instance()
            .set(&COUNTER_KEY, &counter);

        // Extend the TTL of the counter key by one day (86400 seconds)
        let ttl_seconds: u64 = 86_400; // 24 hours
        env.storage().instance().extend_ttl(&COUNTER_KEY, ttl_seconds);

        counter
    }

    /// Return the current value of the counter without modifying it.
    #[allow(clippy::unused_self)]
    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get_or_default::<Symbol, u32>(&COUNTER_KEY)
    }
}
