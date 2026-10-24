#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");
const BUMP_THRESHOLD: u32 = 30 * 17280;
const BUMP_TO: u32 = 120 * 17280;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let value: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0) + 1;
        env.storage().instance().set(&COUNTER, &value);
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);
        value
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}
