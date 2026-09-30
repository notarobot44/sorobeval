#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Bytes, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Entry(u32),
}

const BUMP_THRESHOLD: u32 = 1_000;
const BUMP_TO: u32 = 5_000;

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    /// Store `value` under `key` in persistent storage and extend the
    /// entry's TTL (threshold 1,000 ledgers, extend to 5,000 ledgers).
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage_key = DataKey::Entry(key);
        env.storage().persistent().set(&storage_key, &value);
        env.storage()
            .persistent()
            .extend_ttl(&storage_key, BUMP_THRESHOLD, BUMP_TO);
    }

    /// Return the value stored under `key`, or `None` if absent.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&DataKey::Entry(key))
    }

    /// Extend the TTL of an existing entry with the same parameters
    /// (threshold 1,000, extend to 5,000). No-op if the key does not exist.
    pub fn bump(env: Env, key: u32) {
        let storage_key = DataKey::Entry(key);
        if env.storage().persistent().has(&storage_key) {
            env.storage()
                .persistent()
                .extend_ttl(&storage_key, BUMP_THRESHOLD, BUMP_TO);
        }
    }
}
