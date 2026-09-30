#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env};

/// Bump the entry when it has fewer than this many ledgers left to live.
const TTL_THRESHOLD: u32 = 1_000;
/// Extend the entry so that it lives at least this many ledgers from now.
const TTL_EXTEND_TO: u32 = 5_000;

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    /// Store `value` under `key` in persistent storage and extend the
    /// entry's TTL so it lives at least `TTL_EXTEND_TO` ledgers from now.
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();
        storage.set(&key, &value);
        storage.extend(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    /// Return the value stored under `key`, or `None` if absent.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extend the TTL of an existing entry using the same parameters as
    /// `put`. Does nothing if the key does not exist.
    pub fn bump(env: Env, key: u32) {
        let storage = env.storage().persistent();
        if storage.has(&key) {
            storage.extend(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
        }
    }
}
