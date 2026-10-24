#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const COUNTER: soroban_sdk::Symbol = symbol_short!("COUNTER");

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let current: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let next = current + 1;
        env.storage().instance().set(&COUNTER, &next);
        env.storage().instance().extend_ttl(100, 100);
        next
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}
