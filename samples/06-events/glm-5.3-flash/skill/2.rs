#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, contracttype, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Last,
    Count,
}

#[contractevent(topics = ["record"])]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contract]
pub struct Recorder;

const KEY_LAST: Symbol = symbol_short!("last");
const KEY_COUNT: Symbol = symbol_short!("count");

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env.storage().instance().get(&DataKey::Count).unwrap_or(0) + 1;

        env.storage().instance().set(&DataKey::Last, &value);
        env.storage().instance().set(&DataKey::Count, &count);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&DataKey::Last)
    }
}
