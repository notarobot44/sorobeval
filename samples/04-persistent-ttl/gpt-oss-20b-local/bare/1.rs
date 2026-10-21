#![no_std]

use soroban_sdk::{contractimpl, Env, Bytes, Symbol};

pub struct Persist;

#[contractimpl]
impl Persist {
    /// Stores `value` under `key` in persistent storage and ensures the entry
    /// lives at least 5_000 ledgers from now.
    pub fn put(env: Env, key: u32, value: Bytes) {
        let ledger = env.ledger().current();
        // Store the value
        env.storage()
            .persistent()
            .set(&key, &value);
        // Set expiration to current ledger + 5_000
        env.storage()
            .persistent()
            .set_expiration_ledger(&key, ledger + 5_000);
    }

    /// Retrieves the value stored under `key`, or `None` if it does not exist.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extends the TTL of an existing entry to at least 5_000 ledgers from now.
    /// Does nothing if the key is absent.
    pub fn bump(env: Env, key: u32) {
        let ledger = env.ledger().current();
        // Only extend if the key exists
        if env.storage().persistent().contains(&key) {
            env.storage()
                .persistent()
                .set_expiration_ledger(&key, ledger + 5_000);
        }
    }
}
