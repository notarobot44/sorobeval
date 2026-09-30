#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env};

const TTL_THRESHOLD: u32 = 1_000;
const TTL_EXTEND_TO: u32 = 5_000;

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    /// Stores `value` under `key` in persistent storage and extends the
    /// entry's TTL so that it lives at least 5,000 ledgers from now.
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();
        storage.set(&key, &value);
        storage.extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    /// Returns the stored value for `key`, or `None` if absent.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get::<u32, Bytes>(&key)
    }

    /// Extends the TTL of an existing entry; no-op if the key does not exist.
    pub fn bump(env: Env, key: u32) {
        let storage = env.storage().persistent();
        if storage.has(&key) {
            storage.extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
        }
    }
}
