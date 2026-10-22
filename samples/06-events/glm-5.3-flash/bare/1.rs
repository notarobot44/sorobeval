#![no_std]

use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env, Symbol, Vec};

const KEY: Symbol = symbol_short!("last");
const COUNT: Symbol = symbol_short!("count");

#[contractevent]
struct Record {
    #[topic]
    count: u32,
    value: u32,
}

#[contract]
struct Recorder;

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNT).unwrap_or(0) + 1;
        env.storage().instance().set(&COUNT, &count);
        env.storage().instance().set(&KEY, &value);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get::<_, u32>(&KEY)
    }
}
