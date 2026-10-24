#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

const DAY_IN_LEDGERS: u32 = 17280;
const BUMP_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const BUMP_TO: u32 = 120 * DAY_IN_LEDGERS;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let storage = env.storage().instance();
        let count: u32 = storage.get(&COUNTER).unwrap_or(0) + 1;
        storage.set(&COUNTER, &count);
        storage.extend_ttl(BUMP_THRESHOLD, BUMP_TO);
        count
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}
