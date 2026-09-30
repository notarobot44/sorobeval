#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const TTL_THRESHOLD: u32 = 30 * 17_280;
const TTL_EXTEND_TO: u32 = 120 * 17_280;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let count = Self::get(env).checked_add(1).expect("counter overflow");
        env.storage()
            .instance()
            .set(&symbol_short!("counter"), &count);
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
        count
    }

    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("counter"))
            .unwrap_or(0)
    }
}
