#![no_std]
use soroban_sdk::{contract, contractimpl, env::Env, Bytes, Vec};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();
        storage.set(&key, &value);
        Self::bump_internal(&env, &key);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let storage = env.storage().persistent();
        storage.get(&key)
    }

    pub fn bump(env: Env, key: u32) {
        Self::bump_internal(&env, &key);
    }

    fn bump_internal(env: &Env, key: &u32) {
        let storage = env.storage().persistent();
        if storage.has(key) {
            // Extend TTL to at least 5000 ledgers from now
            // Using a threshold of 1000 and extending to 5000
            storage.extend_ttl(key, 1000, 5000);
        }
    }
}
