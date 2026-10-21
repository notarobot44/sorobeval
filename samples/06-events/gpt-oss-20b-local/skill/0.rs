#![no_std]
use soroban_sdk::{
    contract, contractevent, contractimpl, symbol_short, Address, Env,
};

#[contractevent]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    // Store the new value and publish an event. Return how many records
    // have been stored so far.
    pub fn record(env: Env, value: u32) -> u32 {
        let count_key = symbol_short!("cnt");
        let last_key = symbol_short!("last");

        // Current count (0 if not set yet)
        let mut count = env
            .storage()
            .instance()
            .get(&count_key)
            .unwrap_or(0u32);

        // Increment and store the new count
        count += 1;
        env.storage().instance().set(&count_key, &count);

        // Store the most recent value
        env.storage().instance().set(&last_key, &value);

        // Publish an event with the new count as a topic
        Record { count, value }.publish(&env);

        count
    }

    // Return the last recorded value, if any.
    pub fn last(env: Env) -> Option<u32> {
        let last_key = symbol_short!("last");
        env.storage().instance().get(&last_key)
    }
}
