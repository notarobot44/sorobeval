#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let key = symbol_short!("count");
        let count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        let count = count + 1;
        env.storage().instance().set(&key, &count);
        env.storage().instance().extend_ttl(100000, 200000);
        count
    }

    pub fn get(env: Env) -> u32 {
        let key = symbol_short!("count");
        env.storage().instance().get(&key).unwrap_or(0)
    }
}
