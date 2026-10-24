#![no_std]

use soroban_sdk::{contract, contractimpl, sym, Env};

#[contract]
pub struct Counter;

const KEY: soroban_sdk::Symbol = sym!("COUNTER");

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let val: u32 = env.storage().instance().get(&KEY).unwrap_or(0) + 1;
        env.storage().instance().set(&KEY, &val);
        env.storage().instance().extend_ttl(100, 100);
        val
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&KEY).unwrap_or(0)
    }
}
