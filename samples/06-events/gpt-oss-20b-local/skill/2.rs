#![no_std]
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype,
    symbol_short, Address, Env,
};

#[contracttype]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    // Stores `value` and returns the number of records so far.
    pub fn record(env: Env, value: u32) -> u32 {
        let key = symbol_short!("rec");
        // Get current count (0 if none), increment
        let mut count = env.storage().instance().get(&key).unwrap_or(0u32);
        count += 1;
        // Store the new record value
        env.storage().instance().set(&key, &value);
        // Store the updated count back under same key? We need separate storage for count.
        // Use another key for count to avoid collision with value.
        let count_key = symbol_short!("cnt");
        env.storage()
            .instance()
            .set(&count_key, &count);

        // Publish event
        Record { count, value }.publish(&env);
        count
    }

    pub fn last(env: Env) -> Option<u32> {
        let key = symbol_short!("rec");
        env.storage().instance().get(&key)
    }
}
