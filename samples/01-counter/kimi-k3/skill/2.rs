#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER_KEY: Symbol = symbol_short!("COUNT");
const TTL_THRESHOLD: u32 = 120 * 17280;
const TTL_EXTEND_TO: u32 = 180 * 17280;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let storage = env.storage().instance();
        let count: u32 = storage.get(&COUNTER_KEY).unwrap_or(0);
        let count = count + 1;
        storage.set(&COUNTER_KEY, &count);
        storage.extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
        count
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER_KEY).unwrap_or(0)
    }
}
