#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

const KEY: Symbol = symbol_short!("COUNTER");
const TTL: u32 = 100_000;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let mut val: u32 = env.storage().instance().get(&KEY).unwrap_or(0);
        val += 1;
        env.storage().instance().set(&KEY, &val);
        env.storage().instance().extend_ttl(TTL, TTL);
        val
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&KEY).unwrap_or(0)
    }
}
