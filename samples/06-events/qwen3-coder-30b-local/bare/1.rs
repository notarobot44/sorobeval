#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, env::Env, storage, symbol, Vec};

#[contractevent]
pub struct Record {
    #[topic]
    event_name: symbol,
    #[topic]
    count: u32,
    value: u32,
}

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let mut count = storage::persistent().get::<symbol, u32>(&symbol!("count")).unwrap_or(0);
        count += 1;
        storage::persistent().set(&symbol!("count"), &count);
        storage::persistent().set(&symbol!("last"), &value);
        
        Record {
            event_name: symbol!("record"),
            count,
            value,
        }
        .publish(&env);
        
        count
    }

    pub fn last(env: Env) -> Option<u32> {
        storage::persistent().get::<symbol, u32>(&symbol!("last"))
    }
}
