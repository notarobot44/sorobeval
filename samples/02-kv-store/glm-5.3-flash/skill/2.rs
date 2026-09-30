#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

const DAY_IN_LEDGERS: u32 = 17280;
const BUMP_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const BUMP_TO: u32 = 120 * DAY_IN_LEDGERS;

#[contract]
pub struct Store;

#[contractimpl]
impl Store {
    /// Stores `value` under `key` using instance storage.
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage().instance().set(&key, &value);
        // Instance storage shares a single TTL with the contract instance;
        // extending here keeps all entries alive on writes.
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);
    }

    /// Returns the stored value, or `None` if the key was never set.
    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage().instance().get(&key)
    }

    /// Deletes the key; `get` returns `None` afterwards.
    pub fn remove(env: Env, key: Symbol) {
        env.storage().instance().remove(&key);
    }
}
