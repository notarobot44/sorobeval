#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const DAY_IN_LEDGERS: u32 = 17280;
const BUMP_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const BUMP_TO: u32 = 120 * DAY_IN_LEDGERS;

const COUNTER_KEY: Symbol = symbol_short!("counter");

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increases the counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNTER_KEY).unwrap_or(0);
        let count = count + 1;
        env.storage().instance().set(&COUNTER_KEY, &count);

        // Keep the instance (and its shared storage) alive.
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);

        count
    }

    /// Returns the current counter value without changing it.
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER_KEY).unwrap_or(0)
    }
}
