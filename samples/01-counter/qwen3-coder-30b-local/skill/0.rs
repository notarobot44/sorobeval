#![no_std]
use soroban_sdk::{contract, contractimpl, Env, symbol_short};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&symbol_short!("CNT")).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&symbol_short!("CNT"), &count);
        env.storage().instance().extend_ttl(120 * 17280, 180 * 17280);
        count
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&symbol_short!("CNT")).unwrap_or(0)
    }
}
