#![no_std]

use soroban_sdk::{
    contract::{contractimpl},
    env::Env,
    map::Map,
    Bytes,
    Symbol,
};

pub struct Persist;

#[contractimpl]
impl Persist {
    /// Store `value` under `key` in persistent storage and ensure the entry
    /// has a TTL of at least 5,000 ledgers from now.
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();

        // Write the value.
        storage.set(&key, &value);

        // Set or extend the TTL to 5_000 ledger slots.
        const MIN_TTL: u64 = 5_000;
        storage.set_ttl(&key, MIN_TTL);
    }

    /// Retrieve the value stored under `key`, if any.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let storage = env.storage().persistent();
        storage.get(&key).map(|v| v.into())
    }

    /// Extend the TTL of an existing entry by 5,000 ledger slots.
    /// Does nothing if the key does not exist.
    pub fn bump(env: Env, key: u32) {
        let storage = env.storage().persistent();

        // Only extend if the entry exists.
        if storage.has(&key) {
            const EXTEND_TTL: u64 = 5_000;
            storage.set_ttl(&key, EXTEND_TTL);
        }
    }
}
