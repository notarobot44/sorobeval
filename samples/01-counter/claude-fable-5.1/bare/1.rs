#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

const TTL_THRESHOLD: u32 = 100;
const TTL_EXTEND_TO: u32 = 1000;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let current: u32 = env
            .storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0u32);

        let new_value = current + 1;

        env.storage().instance().set(&COUNTER, &new_value);
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        new_value
    }

    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0u32)
    }
}
