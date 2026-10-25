#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, env::Env, storage, symbol, Vec};

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
        let mut count = storage::persistent::get::<u32>(&env, symbol!("COUNT")).unwrap_or(0);
        count += 1;
        storage::persistent::set(&env, symbol!("COUNT"), &count);
        storage::persistent::set(&env, symbol!("LAST"), &value);
        
        Record {
            name: symbol!("record"),
            count,
            value,
        }
        .publish(&env);
        
        count
    }

    pub fn last(env: Env) -> Option<u32> {
        storage::persistent::get(&env, symbol!("LAST"))
    }
}
