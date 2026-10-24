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
    /// Increase the counter by one and return the new value.
    pub fn increment(env: Env) -> u32 {
        let current: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let next = current.checked_add(1).expect("counter overflow");
        env.storage().instance().set(&COUNTER, &next);
        env.storage()
            .instance()
            .extend_ttl(BUMP_THRESHOLD, BUMP_TO);
        next
    }

    /// Return the current counter value without modifying it.
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}
