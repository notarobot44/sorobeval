#![no_std]

use soroban_sdk::{contractimpl, contractevent, symbol, Env};

#[derive(Clone, Debug)]
pub struct Recorder;

#[contractevent]
pub struct RecordEvent {
    #[topic] pub count: u32,
    pub value: u32,
}

#[contractimpl]
impl Recorder {
    /// Store a value and publish an event.
    pub fn record(env: Env, value: u32) -> u32 {
        // Keys for storage
        let last_key = symbol::short("last");
        let count_key = symbol::short("count");

        // Increment the counter
        let prev_count: Option<u32> = env.storage().get(&count_key);
        let count = prev_count.unwrap_or(0) + 1;
        env.storage().set(&count_key, &count);

        // Store the value as the last recorded
        env.storage().set(&last_key, &value);

        // Publish event
        RecordEvent { count, value }.publish(&env);

        count
    }

    /// Return the most recently recorded value.
    pub fn last(env: Env) -> Option<u32> {
        let last_key = symbol::short("last");
        env.storage().get(&last_key)
    }
}
