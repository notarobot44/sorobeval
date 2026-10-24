#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env, Symbol};

const VALUE: Symbol = symbol_short!("value");
const COUNT: Symbol = symbol_short!("count");

#[contractevent]
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
        let count: u32 = env.storage().instance().get(&COUNT).unwrap_or(0) + 1;
        env.storage().instance().set(&COUNT, &count);
        env.storage().instance().set(&VALUE, &value);
        Record { count, value }.publish(&env);
        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&VALUE)
    }
}
