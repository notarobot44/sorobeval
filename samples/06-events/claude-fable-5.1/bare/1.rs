#![no_std]

use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env, Symbol};

const COUNT: Symbol = symbol_short!("COUNT");
const LAST: Symbol = symbol_short!("LAST");

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
        let storage = env.storage().instance();
        let count: u32 = storage.get::<Symbol, u32>(&COUNT).unwrap_or(0) + 1;
        storage.set(&COUNT, &count);
        storage.set(&LAST, &value);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get::<Symbol, u32>(&LAST)
    }
}
