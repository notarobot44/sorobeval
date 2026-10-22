#![no_std]

use soroban_sdk::{contract, contractevent, contractimpl, contracttype, symbol_short, Env, Symbol};

const KEY: Symbol = symbol_short!("value");
const COUNT: Symbol = symbol_short!("count");

#[contractevent(topics = ["record"], data_format = "single-value")]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Value,
    Count,
}

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env.storage().instance().get(&DataKey::Count).unwrap_or(0) + 1;
        env.storage().instance().set(&DataKey::Count, &count);
        env.storage().instance().set(&DataKey::Value, &value);
        env.storage().instance().extend_ttl(120 * 17280, 180 * 17280);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&DataKey::Value)
    }
}
