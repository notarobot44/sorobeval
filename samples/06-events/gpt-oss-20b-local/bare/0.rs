#![no_std]

use soroban_sdk::{
    contract, contractimpl,
    Env,
    Symbol,
};

#[derive(Clone)]
pub struct Recorder;

#[contract]
pub struct Contract;

#[derive(Clone)]
#[contractevent]
struct Record {
    #[topic]
    count: u32,
    #[data]
    value: u32,
}

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        // Key for the stored value
        let val_key = Symbol::short("value");
        env.storage().instance().set(&val_key, &value);

        // Key for the count of records
        let cnt_key = Symbol::short("count");
        let current = match env.storage().instance().get::<_, u32>(&cnt_key) {
            Some(c) => c,
            None => 0u32,
        };
        let new_count = current + 1;
        env.storage()
            .instance()
            .set(&cnt_key, &new_count);

        // Publish the event
        Record { count: new_count, value }.publish(&env);

        new_count
    }

    pub fn last(env: Env) -> Option<u32> {
        let val_key = Symbol::short("value");
        env.storage().instance().get::<_, u32>(&val_key)
    }
}
