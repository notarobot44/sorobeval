#![no_std]

use soroban_sdk::{
    contractimpl,
    Env,
    Symbol,
    contractevent,
};

#[derive(Clone)]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    /// Record a new value, publish an event and return the total count.
    pub fn record(env: Env, value: u32) -> u32 {
        // Keys for instance storage
        let count_key = Symbol::short("count");
        let value_key = Symbol::short("value");

        // Get current count (default 0)
        let mut count = env.storage().instance().get::<Symbol, u32>(&count_key).unwrap_or(0);
        count += 1;

        // Store new count and value
        env.storage()
            .instance()
            .set(&count_key, &count);
        env.storage()
            .instance()
            .set(&value_key, &value);

        // Publish event
        Record { count, value }.publish(&env);

        count
    }

    /// Return the most recently recorded value.
    pub fn last(env: Env) -> Option<u32> {
        let value_key = Symbol::short("value");
        env.storage().instance().get::<Symbol, u32>(&value_key)
    }
}

#[contractevent]
pub struct Record {
    #[topic] count: u32,
    value: u32,
}
