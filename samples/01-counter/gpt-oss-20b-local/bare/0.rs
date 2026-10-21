#![no_std]

use soroban_sdk::{contractimpl, contracttype, Env, Symbol};

#[derive(Clone)]
pub struct Counter;

#[contracttype]
struct CounterKey(Symbol);

const COUNTER_KEY: CounterKey = CounterKey(Symbol::short("counter"));

#[contractimpl]
impl Counter {
    /// Increments the persistent counter by one and returns the new value.
    /// The counter starts at 0. Extends the instance TTL on every increment.
    pub fn increment(env: Env) -> u32 {
        // Retrieve current value (default to 0 if not set)
        let mut val = env
            .storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(u32::from(0));

        // Increment and store the new value
        val += 1;
        env.storage().instance().set(&COUNTER_KEY, &val);

        // Extend the instance TTL (e.g., by 10,000 seconds)
        // Note: Adjust duration as appropriate for your deployment.
        let extend_duration = soroban_sdk::Duration::from_seconds(10_000);
        env.storage()
            .instance()
            .extend_ttl(&COUNTER_KEY, &extend_duration);

        val
    }

    /// Returns the current value of the counter without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(u32::from(0))
    }
}
