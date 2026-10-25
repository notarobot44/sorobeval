#![no_std]
use soroban_sdk::{Bytes, Env, Symbol};

#[soroban_sdk::contract]
pub struct Persist;

#[soroban_sdk::contractimpl]
impl Persist {
    /// Store `value` under `key` in persistent storage and extend its TTL.
    ///
    /// The entry is guaranteed to live at least 5 000 ledgers from now.
    pub fn put(env: Env, key: u32, value: Bytes) {
        env.storage()
            .persistent()
            .set(&key, &value);

        // Extend the TTL only if it is below the threshold.
        const THRESHOLD: u32 = 1_000;
        const EXTEND_TO: u32 = 5_000;
        env.storage()
            .persistent()
            .extend_ttl(&key, THRESHOLD, EXTEND_TO);
    }

    /// Return the value stored under `key`, or `None` if no entry exists.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage()
            .persistent()
            .get(&key)
            .map(|v| v.into_val(&env))
    }

    /// Extend the TTL of an existing entry with the same parameters as `put`.
    ///
    /// Does nothing if the key does not exist.
    pub fn bump(env: Env, key: u32) {
        // Only attempt to extend if we can read the value (i.e. it exists).
        if env.storage().persistent().has(&key) {
            const THRESHOLD: u32 = 1_000;
            const EXTEND_TO: u32 = 5_000;
            env.storage()
                .persistent()
                .extend_ttl(&key, THRESHOLD, EXTEND_TO);
        }
    }
}
