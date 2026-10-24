#![no_std]

use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env};

#[contract]
pub struct Recorder;

#[contractevent(topics = ["record"])]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let count = env
            .storage()
            .instance()
            .get::<_, u32>(&symbol_short!("count"))
            .unwrap_or(0)
            .checked_add(1)
            .expect("record count overflow");

        env.storage()
            .instance()
            .set(&symbol_short!("last"), &value);
        env.storage()
            .instance()
            .set(&symbol_short!("count"), &count);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&symbol_short!("last"))
    }
}
