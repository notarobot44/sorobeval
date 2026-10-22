#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const KEY: Symbol = symbol_short!("COUNT");
const TTL_THRESHOLD: u32 = 100;
const TTL_EXTEND_TO: u32 = 1000;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let storage = env.storage().instance();
        let mut value: u32 = storage.get(&KEY).unwrap_or(0);
        value += 1;
        storage.set(&KEY, &value);
        storage.extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
        value
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&KEY).unwrap_or(0)
    }
}
