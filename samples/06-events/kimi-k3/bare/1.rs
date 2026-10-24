#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env};

#[contract]
pub struct Recorder;

#[contractevent]
#[derive(Clone, Debug)]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

const VALUE: soroban_sdk::Symbol = symbol_short!("value");
const COUNT: soroban_sdk::Symbol = symbol_short!("count");

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let storage = env.storage().instance();
        let count: u32 = storage.get(&COUNT).unwrap_or(0) + 1;
        storage.set(&VALUE, &value);
        storage.set(&COUNT, &count);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&VALUE)
    }
}
