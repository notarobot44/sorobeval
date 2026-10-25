#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, env::Events, symbol, Env, Storage};

#[contractevent]
pub struct Record {
    #[topic]
    event_name: symbol,
    #[topic]
    count: u32,
    data: u32,
}

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let mut count = env.storage().get::<symbol, u32>(&symbol!("count")).unwrap_or(0);
        count += 1;
        env.storage().set(&symbol!("count"), &count);
        env.storage().set(&symbol!("last"), &value);
        
        Record {
            event_name: symbol!("record"),
            count,
            data: value,
        }
        .publish(&env);
        
        count
    }
    
    pub fn last(env: Env) -> Option<u32> {
        env.storage().get::<symbol, u32>(&symbol!("last"))
    }
}
