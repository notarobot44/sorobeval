#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env};

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
    pub fn record(env: Env, value: u32) -> u32 {
        let key = symbol_short!("rec");
        let count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        let new_count = count + 1;
        env.storage().instance().set(&key, &new_count);

        Record { count: new_count, value }.publish(&env);
        new_count
    }

    pub fn last(env: Env) -> Option<u32> {
        let key = symbol_short!("rec");
        env.storage().instance().get(&key)
    }
}
