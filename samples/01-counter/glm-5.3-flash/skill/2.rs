#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

#[contract]
pub struct Counter;

const DAY_IN_LEDGERS: u32 = 17280;

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
        env.storage()
            .instance()
            .extend_ttl(30 * DAY_IN_LEDGERS, 120 * DAY_IN_LEDGERS);
        count
    }

    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("count"))
            .unwrap_or(0)
    }
}
