#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Bytes};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    /// Stores `value` under `key` in persistent storage and extends the entry's TTL
    /// so it lives at least 5 000 ledgers from now (threshold 1 000).
    pub fn put(env: Env, key: u32, value: Bytes) {
        let pers = env.storage().persistent();
        pers.set(&key, &value);
        // Extend the TTL if it's below the threshold.
        pers.extend_ttl(&key, 1_000, 5_000);
    }

    /// Returns the stored value for `key`, or `None` if it does not exist.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extends the TTL of an existing entry with the same parameters as `put`.
    /// Does nothing if the key is absent.
    pub fn bump(env: Env, key: u32) {
        let pers = env.storage().persistent();
        if pers.has(&key) {
            pers.extend_ttl(&key, 1_000, 5_000);
        }
    }
}
