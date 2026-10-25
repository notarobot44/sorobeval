#![no_std]

use soroban_sdk::{
    contract, contractevent, contractimpl, symbol_short, Env,
};

#[derive(Clone)]
enum DataKey {
    Last,
    Count,
}

#[contract]
pub struct Recorder;

#[contractevent]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contractimpl]
impl Recorder {
    /// Stores `value`, publishes an event, and returns the number of records so far.
    pub fn record(env: Env, value: u32) -> u32 {
        // Increment the counter
        let current = env.storage().instance().get::<_, u32>(&DataKey::Count).unwrap_or(0);
        let new_count = current + 1;
        env.storage()
            .instance()
            .set(&DataKey::Count, &new_count);

        // Store the latest value
        env.storage().instance().set(&DataKey::Last, &value);

        // Publish the event
        Record {
            count: new_count,
            value,
        }
        .publish(&env);

        new_count
    }

    /// Returns the most recently recorded value, if any.
    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get::<_, u32>(&DataKey::Last)
    }
}
