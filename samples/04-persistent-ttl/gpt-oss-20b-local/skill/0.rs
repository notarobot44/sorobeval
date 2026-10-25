#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Bytes};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    /// Store `value` under `key` in persistent storage and ensure the entry
    /// lives at least 5 000 ledgers from now. If the key already exists,
    /// its value is overwritten.
    pub fn put(env: Env, key: u32, value: Bytes) {
        // Persist the value.
        env.storage()
            .persistent()
            .set(&key, &value);

        // Extend TTL: threshold 1 000, extend to 5 000 ledgers.
        const THRESHOLD: u32 = 1_000;
        const EXTEND_TO: u32 = 5_000;
        env.storage()
            .persistent()
            .extend_ttl(&key, THRESHOLD, EXTEND_TO);
    }

    /// Return the value stored under `key`, or `None` if no entry exists.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extend the TTL of an existing entry with the same parameters used in
    /// `put`. Does nothing if the key does not exist.
    pub fn bump(env: Env, key: u32) {
        // Only extend if the entry exists; otherwise ignore.
        if env.storage().persistent().has(&key) {
            const THRESHOLD: u32 = 1_000;
            const EXTEND_TO: u32 = 5_000;
            env.storage()
                .persistent()
                .extend_ttl(&key, THRESHOLD, EXTEND_TO);
        }
    }
}
