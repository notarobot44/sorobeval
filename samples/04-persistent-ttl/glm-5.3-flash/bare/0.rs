#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct Persist;

const TTL_THRESHOLD: u32 = 1_000;
const TTL_EXTEND_TO: u32 = 5_000;

#[contractimpl]
impl Persist {
    /// Stores `value` under `key` in persistent storage and extends the
    /// entry's TTL so that it lives at least 5,000 ledgers from now.
    pub fn put(env: Env, key: u32, value: Bytes) {
        env.storage().persistent().set(&key, &value);
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    /// Returns the value stored under `key`, or `None` if absent.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extends the TTL of an existing entry; does nothing if the key
    /// does not exist.
    pub fn bump(env: Env, key: u32) {
        if env.storage().persistent().has(&key) {
            env.storage()
                .persistent()
                .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
        }
    }
}
