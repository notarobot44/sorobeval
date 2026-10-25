#![no_std]
use soroban_sdk::{Bytes, Env, Symbol};

#[soroban_sdk::contract]
pub struct Persist;

#[soroban_sdk::contractimpl]
impl Persist {
    /// Store `value` under `key` in persistent storage and extend its TTL
    /// to at least 5 000 ledgers from now (threshold 1 000).
    pub fn put(env: Env, key: u32, value: Bytes) {
        // Persist the value
        env.storage()
            .persistent()
            .set(&key, &value);

        // Extend TTL if current TTL is below threshold.
        // Threshold = 1 000 ledgers, extend to = 5 000 ledgers.
        const THRESHOLD: u32 = 1_000;
        const EXTEND_TO: u32 = 5_000;
        env.storage()
            .persistent()
            .extend_ttl(&key, THRESHOLD, EXTEND_TO);
    }

    /// Return the value stored under `key`, or `None` if it does not exist.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extend the TTL of an existing entry with the same parameters
    /// (threshold = 1 000, extend_to = 5 000). Does nothing if the key
    /// does not exist.
    pub fn bump(env: Env, key: u32) {
        // Only attempt to extend if the key exists.
        if env.storage().persistent().has(&key) {
            const THRESHOLD: u32 = 1_000;
            const EXTEND_TO: u32 = 5_000;
            env.storage()
                .persistent()
                .extend_ttl(&key, THRESHOLD, EXTEND_TO);
        }
    }
}
