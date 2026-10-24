#![no_std]

use soroban_sdk::{contract, contractevent, contracttype, env, symbol_short, Env, Symbol};

#[contractevent]
struct Record {
    #[topic]
    name: Symbol,
    #[topic]
    count: u32,
    value: u32,
}

#[contracttype]
enum DataKey {
    Value,
    Count,
}

#[contract]
pub struct Recorder;

#[cfg_attr(test, allow(dead_code))]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env
            .storage()
            .instance()
            .get::<DataKey, u32>(&DataKey::Count)
            .unwrap_or(0)
            + 1;

        env.storage().instance().set(&DataKey::Value, &value);
        env.storage().instance().set(&DataKey::Count, &count);

        Record {
            name: symbol_short!("record"),
            count,
            value,
        }
        .publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get::<DataKey, u32>(&DataKey::Value)
    }
}
