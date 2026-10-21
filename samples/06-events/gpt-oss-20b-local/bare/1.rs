#![no_std]

use soroban_sdk::{
    contract::{contracterror, contractimpl},
    contracttype,
    Env,
};

#[derive(Clone)]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    /// Store the given `value`, publish an event and return the total number of records.
    pub fn record(env: Env, value: u32) -> u32 {
        // Retrieve current count
        let count_key = Symbol::short("count");
        let rec_count = env.storage().instance().get::<Symbol, u32>(count_key).unwrap_or(0);

        // Store the new value under a short key
        let val_key = Symbol::short("rec");
        env.storage()
            .instance()
            .set::<Symbol, u32>(val_key.clone(), &value);

        // Increment and store the count
        let new_count = rec_count + 1;
        env.storage().instance().set::<Symbol, u32>(count_key, &new_count);

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
        let val_key = Symbol::short("rec");
        env.storage().instance().get::<Symbol, u32>(val_key)
    }
}

/// Event type for a record operation.
#[contractevent]
pub struct Record {
    #[topic] count: u32,
    value: u32,
}
