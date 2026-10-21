#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env,
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Value,
    Count,
}

#[contractevent]
struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    /// Stores `value`, emits an event, and returns the total number of records.
    pub fn record(env: Env, value: u32) -> u32 {
        // Increment the counter
        let count_key = DataKey::Count;
        let new_count = match env.storage().instance().get(&count_key) {
            Some(c) => c + 1,
            None => 1,
        };
        env.storage()
            .instance()
            .set(&DataKey::Count, &new_count);

        // Store the value
        env.storage().instance().set(&DataKey::Value, &value);

        // Emit event
        Record {
            count: new_count,
            value,
        }
        .publish(&env);

        new_count
    }

    /// Returns the most recently recorded value, if any.
    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&DataKey::Value)
    }
}
