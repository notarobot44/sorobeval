#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const TTL_THRESHOLD: u32 = 30 * 17_280;
const TTL_EXTEND_TO: u32 = 120 * 17_280;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let key = symbol_short!("counter");
        let count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        let next = count.checked_add(1).expect("counter overflow");

        env.storage().instance().set(&key, &next);
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);

        next
    }

    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("counter"))
            .unwrap_or(0)
    }
}
