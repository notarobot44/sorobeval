#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, contracttype, env::Events, symbol, Env, Storage};

#[contractevent]
pub struct Record {
    #[topic]
    event_name: symbol,
    #[topic]
    count: u32,
    value: u32,
}

#[contracttype]
pub enum DataKey {
    Count,
    Value(u32),
}

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let mut count = Storage::get(&env, &DataKey::Count).unwrap_or(0);
        count += 1;
        Storage::set(&env, &DataKey::Count, &count);
        Storage::set(&env, &DataKey::Value(count - 1), &value);
        
        Record {
            event_name: symbol!("record"),
            count,
            value,
        }
        .publish(&env);
        
        count
    }

    pub fn last(env: Env) -> Option<u32> {
        let count = Storage::get(&env, &DataKey::Count).unwrap_or(0);
        if count == 0 {
            None
        } else {
            Storage::get(&env, &DataKey::Value(count - 1))
        }
    }
}
