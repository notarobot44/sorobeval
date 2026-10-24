#![no_std]

use soroban_sdk::{contract, contractclient, contractevent, contractimpl, contracttype, Env, Vec};

#[contracttype]
pub enum DataKey {
    Last,
    Count,
}

#[contractevent]
struct Record {
    #[topic]
    count: u32,
    value: u32,
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

        env.storage().instance().set(&DataKey::Last, &value);
        env.storage().instance().set(&DataKey::Count, &count);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&DataKey::Last)
    }
}
