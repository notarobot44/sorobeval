#![no_std]

use soroban_sdk::{
    contract, contractevent, contractimpl, symbol_short, Env,
};

/// Event published when a value is recorded.
#[contractevent]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

/// The contract that records values and publishes an event for each record.
#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    /// Stores `value` in instance storage, publishes a `Record` event
    /// and returns the number of records so far (starting at 1).
    pub fn record(env: Env, value: u32) -> u32 {
        // Keys for count and last value.
        let cnt_key = symbol_short!("cnt");
        let val_key = symbol_short!("val");

        // Get current count, defaulting to 0.
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&cnt_key)
            .unwrap_or(0);
        count += 1;

        // Store new count and the value.
        env.storage().instance().set(&cnt_key, &count);
        env.storage().instance().set(&val_key, &value);

        // Publish event.
        Record { count, value }.publish(&env);

        count
    }

    /// Returns the most recently recorded value, if any.
    pub fn last(env: Env) -> Option<u32> {
        let val_key = symbol_short!("val");
        env.storage().instance().get(&val_key)
    }
}
