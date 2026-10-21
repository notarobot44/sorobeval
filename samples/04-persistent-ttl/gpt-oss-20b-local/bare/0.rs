#![no_std]

use soroban_sdk::{Bytes, Env, Symbol};

pub struct Persist;

impl Persist {
    /// Store `value` under `key` in persistent storage and set the entry's TTL to at least 5 000 ledgers.
    pub fn put(env: &Env, key: u32, value: Bytes) {
        // Store the value
        env.storage()
            .persistent()
            .set(&Symbol::new("kv").into(), &key, &value);

        // Extend TTL to 5,000 ledgers from now
        let ttl = soroban_sdk::Expiration::from_ledger_age(5000);
        env.storage()
            .persistent()
            .set_expiration(&Symbol::new("kv").into(), &key, &ttl);
    }

    /// Retrieve the value stored under `key`, if any.
    pub fn fetch(env: &Env, key: u32) -> Option<Bytes> {
        env.storage()
            .persistent()
            .get(&Symbol::new("kv").into(), &key)
    }

    /// Extend the TTL of an existing entry by 5 000 ledgers from now. Does nothing if the key does not exist.
    pub fn bump(env: &Env, key: u32) {
        // Check existence
        if env.storage()
            .persistent()
            .contains(&Symbol::new("kv").into(), &key)
        {
            let ttl = soroban_sdk::Expiration::from_ledger_age(5000);
            env.storage()
                .persistent()
                .set_expiration(&Symbol::new("kv").into(), &key, &ttl);
        }
    }
}
