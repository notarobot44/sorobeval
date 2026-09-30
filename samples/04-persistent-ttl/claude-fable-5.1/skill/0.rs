#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

const TTL_THRESHOLD: u32 = 1_000;
const TTL_EXTEND_TO: u32 = 5_000;

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();
        storage.set(&key, &value);
        storage.extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    pub fn bump(env: Env, key: u32) {
        let storage = env.storage().persistent();
        if storage.has(&key) {
            storage.extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
        }
    }
}
