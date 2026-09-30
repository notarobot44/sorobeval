#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

const BUMP_THRESHOLD: u32 = 1_000;
const BUMP_TO: u32 = 5_000;

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    /// Stores `value` under `key` in persistent storage and extends the
    /// entry's TTL (threshold 1,000, extend to 5,000).
    pub fn put(env: Env, key: u32, value: Bytes) {
        env.storage().persistent().set(&key, &value);
        env.storage()
            .persistent()
            .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TO);
    }

    /// Returns the value stored under `key`, or `None` if absent.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extends the TTL of an existing entry (threshold 1,000, extend to
    /// 5,000). Does nothing if the key does not exist.
    pub fn bump(env: Env, key: u32) {
        if env.storage().persistent().has(&key) {
            env.storage()
                .persistent()
                .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TO);
        }
    }
}
