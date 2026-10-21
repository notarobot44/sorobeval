#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Bytes};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    /// Store `value` under `key` in persistent storage and extend its TTL.
    pub fn put(env: Env, key: u32, value: Bytes) {
        // Write the new value
        env.storage()
            .persistent()
            .set(&key, &value);

        // Extend the entry's TTL so it lives at least 5 000 ledgers from now.
        // The entry will be extended only if its current TTL is below 1 000,
        // otherwise nothing changes. This keeps storage rent low for active keys.
        env.storage()
            .persistent()
            .extend_ttl(&key, 1_000, 5_000);
    }

    /// Retrieve the value stored under `key`, or `None` if it does not exist.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extend the TTL of an existing entry with the same parameters as in `put`.
    /// Does nothing if the key is absent.
    pub fn bump(env: Env, key: u32) {
        // Only extend if the key exists; otherwise we avoid creating a new
        // storage entry inadvertently.
        if env.storage().persistent().has(&key) {
            env.storage()
                .persistent()
                .extend_ttl(&key, 1_000, 5_000);
        }
    }
}
