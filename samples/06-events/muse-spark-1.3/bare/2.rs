#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env};

#[contract]
pub struct Recorder;

#[contractevent(data_format = "single-value")]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let count_key = symbol_short!("count");
        let last_key = symbol_short!("last");
        let count: u32 = env.storage().instance().get(&count_key).unwrap_or(0);
        let count = count + 1;
        env.storage().instance().set(&count_key, &count);
        env.storage().instance().set(&last_key, &value);
        Record { count, value }.publish(&env);
        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&symbol_short!("last"))
    }
}
