#![no_std]

use soroban_sdk::{contractimpl, Bytes, Env};

pub struct Persist;

#[contractimpl]
impl Persist {
    /// Stores `value` under `key` in persistent storage and extends the entry's TTL
    /// so that it lives at least 5_000 ledgers from now (threshold 1_000, extend to 5_000).
    pub fn put(env: Env, key: u32, value: Bytes) {
        // Persist the value with a TTL of 5_000 ledgers.
        let ttl = 5_000;
        env.storage()
            .persistent()
            .set_with_ttl(&key, &value, ttl);
    }

    /// Returns the value stored under `key` or `None` if it does not exist.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        match env.storage().persistent().get::<u32, Bytes>(&key) {
            Some(val) => Some(val),
            None => None,
        }
    }

    /// Extends the TTL of an existing entry with the same parameters; does nothing if the key does not exist.
    pub fn bump(env: Env, key: u32) {
        if let Some(existing_value) = env.storage().persistent().get::<u32, Bytes>(&key) {
            // Re‑store the value to extend its TTL to 5_000 ledgers from now.
            let ttl = 5_000;
            env.storage()
                .persistent()
                .set_with_ttl(&key, &existing_value, ttl);
        }
    }
}
