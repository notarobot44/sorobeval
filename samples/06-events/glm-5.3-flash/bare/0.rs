#![no_std]

use soroban_sdk::{contract, contractclient, contracttype, contractimpl, symbol_short, Env, Symbol, Vec, Address};

#[contract]
pub struct Recorder;

const KEY_LAST: Symbol = symbol_short!("last");
const KEY_COUNT: Symbol = symbol_short!("count");

#[contractevent(name = "record")]
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
            .get(&KEY_COUNT)
            .unwrap_or(0)
            + 1;

        env.storage().instance().set(&KEY_LAST, &value);
        env.storage().instance().set(&KEY_COUNT, &count);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&KEY_LAST)
    }
}
