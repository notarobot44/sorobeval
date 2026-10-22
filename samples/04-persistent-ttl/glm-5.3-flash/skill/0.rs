#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

const TTL_THRESHOLD: u32 = 1_000;
const TTL_EXTEND_TO: u32 = 5_000;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Value(u32),
}

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();
        storage.set(&DataKey::Value(key), &value);
        storage.extend_ttl(&DataKey::Value(key), TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&DataKey::Value(key))
    }

    pub fn bump(env: Env, key: u32) {
        let storage = env.storage().persistent();
        let storage_key = DataKey::Value(key);
        if storage.has(&storage_key) {
            storage.extend_ttl(&storage_key, TTL_THRESHOLD, TTL_EXTEND_TO);
        }
    }
}
