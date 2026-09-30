#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

const DAY_IN_LEDGERS: u32 = 17280;
const BUMP_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const BUMP_TO: u32 = 120 * DAY_IN_LEDGERS;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increases the counter by one and returns the new value.
    pub fn increment(env: Env) -> u32 {
        let count: u32 = env
            .storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0);

        let count = count + 1;
        env.storage().instance().set(&symbol_short!("COUNT"), &count);

        // Keep the instance (and its storage) alive.
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);

        count
    }

    /// Returns the current counter value without changing it.
    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0)
    }
}
