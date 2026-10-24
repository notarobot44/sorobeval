#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNT: Symbol = symbol_short!("count");

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNT).unwrap_or(0);
        let new_count = count + 1;
        env.storage().instance().set(&COUNT, &new_count);
        env.storage().instance().extend_ttl(50, 100);
        new_count
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNT).unwrap_or(0)
    }
}
