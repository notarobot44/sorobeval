#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Store;

// Instance storage shares one TTL with the contract instance; bump it on
// writes so the store isn't archived while in active use.
const DAY_IN_LEDGERS: u32 = 17280;
const BUMP_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const BUMP_TO: u32 = 120 * DAY_IN_LEDGERS;

#[contractimpl]
impl Store {
    /// Stores `value` under `key` in instance storage.
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage().instance().set(&key, &value);
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);
    }

    /// Returns the value stored under `key`, or `None` if it was never set.
    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage().instance().get(&key)
    }

    /// Deletes `key`; `get` returns `None` afterwards.
    pub fn remove(env: Env, key: Symbol) {
        env.storage().instance().remove(&key);
    }
}
