#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env};

#[contract]
pub struct Recorder;

#[contractevent(data_format = "single-value")]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env
            .storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0u32)
            + 1;
        env.storage()
            .instance()
            .set(&symbol_short!("COUNT"), &count);
        env.storage().instance().set(&symbol_short!("LAST"), &value);
        Record { count, value }.publish(&env);
        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&symbol_short!("LAST"))
    }
}
