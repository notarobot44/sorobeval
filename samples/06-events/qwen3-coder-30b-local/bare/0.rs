#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, env::Events, symbol, Env, Storage};

#[contractevent]
pub struct Record {
    #[topic]
    name: symbol,
    #[topic]
    count: u32,
    value: u32,
}

pub struct Recorder;

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let mut count = env.storage().get(&symbol!("count")).unwrap_or(0u32);
        count += 1;
        env.storage().set(&symbol!("count"), count);
        env.storage().set(&symbol!("last"), value);
        
        Record::publish(&env, symbol!("record"), count, value);
        
        count
    }
    
    pub fn last(env: Env) -> Option<u32> {
        env.storage().get(&symbol!("last"))
    }
}
