#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env};

/// Minimum remaining lifetime (in ledgers) below which an entry's TTL is
/// extended.
const TTL_THRESHOLD: u32 = 1_000;

/// Target lifetime (in ledgers, counted from now) an entry should have
/// after being extended.
const TTL_TARGET: u32 = 5_000;

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    /// Store `value` under `key` in persistent storage and ensure the
    /// entry lives at least `TTL_TARGET` ledgers from now.
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();
        storage.set(&key, &value);
        storage.extend_ttl(&key, TTL_THRESHOLD, TTL_TARGET);
    }

    /// Return the value stored under `key`, or `None` if it does not exist.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extend the TTL of an existing entry using the same parameters as
    /// `put`; does nothing if the key does not exist.
    pub fn bump(env: Env, key: u32) {
        let storage = env.storage().persistent();
        if storage.has(&key) {
            storage.extend_ttl(&key, TTL_THRESHOLD, TTL_TARGET);
        }
    }
}
