#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const BUMP_THRESHOLD: u32 = 120 * 17280;
const BUMP_TO: u32 = 180 * 17280;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let key = symbol_short!("count");
        let count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        let new_count = count + 1;
        env.storage().instance().set(&key, &new_count);
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);
        new_count
    }

    pub fn get(env: Env) -> u32 {
        let key = symbol_short!("count");
        env.storage().instance().get(&key).unwrap_or(0)
    }
}
