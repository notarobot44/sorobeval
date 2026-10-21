#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Bytes};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    /// Stores `value` under `key` in persistent storage and extends its TTL
    /// to at least 5 000 ledgers from now (threshold 1 000).
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();
        storage.set(&key, &value);
        // Extend TTL only if current TTL is below the threshold.
        storage.extend_ttl(&key, 1_000u32, 5_000u32);
    }

    /// Returns the stored value or `None` if the key does not exist.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extends the TTL of an existing entry with the same parameters as `put`.
    /// Does nothing if the key is absent.
    pub fn bump(env: Env, key: u32) {
        let storage = env.storage().persistent();
        if storage.has(&key) {
            storage.extend_ttl(&key, 1_000u32, 5_000u32);
        }
    }
}
