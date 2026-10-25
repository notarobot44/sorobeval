#![no_std]

use soroban_sdk::{Bytes, Env, Symbol};

#[derive(Default)]
pub struct Persist;

impl Persist {
    /// Store a value with the given key and set its TTL to at least 5_000 ledgers.
    pub fn put(env: Env, key: u32, value: Bytes) {
        // Persist the value
        env.storage()
            .persistent()
            .set(&key.to_le_bytes(), &value);

        // Extend the TTL. If the current TTL is less than 5_000, set it to 5_000.
        // The SDK's `extend_ttl` method accepts a duration in ledgers.
        let ttl = 5_000u32;
        env.storage().persistent().extend_ttl(&key.to_le_bytes(), ttl);
    }

    /// Retrieve the value for a key, if it exists.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        match env
            .storage()
            .persistent()
            .get::<_, Bytes>(&key.to_le_bytes())
        {
            Some(v) => Some(v),
            None => None,
        }
    }

    /// Extend the TTL of an existing entry to at least 5_000 ledgers.
    pub fn bump(env: Env, key: u32) {
        // Only attempt to extend if the key exists
        if env.storage().persistent().has(&key.to_le_bytes()) {
            let ttl = 5_000u32;
            env.storage().persistent().extend_ttl(&key.to_le_bytes(), ttl);
        }
    }
}
