#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Bytes, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Value(u32),
}

const BUMP_THRESHOLD: u32 = 1_000;
const BUMP_TO: u32 = 5_000;

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage_key = DataKey::Value(key);
        env.storage().persistent().set(&storage_key, &value);
        env.storage()
            .persistent()
            .extend_ttl(&storage_key, BUMP_THRESHOLD, BUMP_TO);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&DataKey::Value(key))
    }

    pub fn bump(env: Env, key: u32) {
        let storage_key = DataKey::Value(key);
        if env.storage().persistent().has(&storage_key) {
            env.storage()
                .persistent()
                .extend_ttl(&storage_key, BUMP_THRESHOLD, BUMP_TO);
        }
    }
}
