#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const BUMP_THRESHOLD: u32 = 120 * 17280;
const BUMP_TO: u32 = 180 * 17280;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let count: u32 = env
            .storage()
            .instance()
            .get(&symbol_short!("count"))
            .unwrap_or(0);
        let count = count + 1;
        env.storage().instance().set(&symbol_short!("count"), &count);
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);
        count
    }

    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("count"))
            .unwrap_or(0)
    }
}
