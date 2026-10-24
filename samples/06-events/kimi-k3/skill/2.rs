#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Address, Env, Symbol};

const VALUE_KEY: Symbol = symbol_short!("VALUE");
const COUNT_KEY: Symbol = symbol_short!("COUNT");

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
        let count: u32 = env
            .storage()
            .instance()
            .get(&COUNT_KEY)
            .unwrap_or(0u32)
            .checked_add(1)
            .expect("count overflow");

        env.storage().instance().set(&VALUE_KEY, &value);
        env.storage().instance().set(&COUNT_KEY, &count);
        env.storage().instance().extend_ttl(120 * 17280, 180 * 17280);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&VALUE_KEY)
    }
}
