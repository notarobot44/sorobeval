#![no_std]

use soroban_sdk::{
    contractimpl,
    contractevent,
    symbol::Symbol,
    Env,
};

#[contractevent(name = "record")]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

pub struct Recorder;

#[contractimpl]
impl Recorder {
    /// Store a new record, publish an event and return the total number of records.
    pub fn record(env: Env, value: u32) -> u32 {
        // Key for storing the current count
        let count_key = Symbol::short("count");
        // Retrieve existing count or default to 0
        let current_count = env.storage().instance().get(&count_key).unwrap_or(0u32);
        let new_count = current_count + 1;

        // Store the new count and value
        env.storage()
            .instance()
            .set(&count_key, &new_count);

        let value_key = Symbol::short("value");
        env.storage().instance().set(&value_key, &value);

        // Publish event
        Record {
            count: new_count,
            value,
        }
        .publish(&env);

        new_count
    }

    /// Return the most recently recorded value.
    pub fn last(env: Env) -> Option<u32> {
        let value_key = Symbol::short("value");
        env.storage().instance().get(&value_key).ok()
    }
}
