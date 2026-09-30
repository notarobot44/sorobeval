#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();
        storage.set(&key, &value);
        storage.extend_ttl(&key, 1_000, 5_000);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    pub fn bump(env: Env, key: u32) {
        let storage = env.storage().persistent();
        if storage.has(&key) {
            storage.extend_ttl(&key, 1_000, 5_000);
        }
    }
}
