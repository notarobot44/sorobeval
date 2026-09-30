#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env};

#[contract]
pub struct Recorder;

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
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
            .get(&symbol_short!("count"))
            .unwrap_or(0u32)
            + 1;
        env.storage()
            .instance()
            .set(&symbol_short!("count"), &count);
        env.storage().instance().set(&symbol_short!("last"), &value);
        env.storage().instance().extend_ttl(50, 1000);
        Record { count, value }.publish(&env);
        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&symbol_short!("last"))
    }
}
