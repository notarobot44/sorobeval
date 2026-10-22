#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, contracttype, symbol_short, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Value,
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

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::Count)
            .unwrap_or(0)
            + 1;

        env.storage().instance().set(&DataKey::Value, &value);
        env.storage().instance().set(&DataKey::Count, &count);
        env.storage()
            .instance()
            .extend_ttl(120 * 17280, 180 * 17280);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&symbol_short!("value"))
    }
}
