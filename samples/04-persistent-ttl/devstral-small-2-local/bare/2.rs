#![no_std]

use soroban_sdk::{contractimpl, contracttype, Bytes, Env};

#[contracttype]
pub enum DataKey {
    Value(u32),
}

pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let threshold = 1000;
        let min_ttl = 5000;
        env.storage().persistent().set(&DataKey::Value(key), &value);
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Value(key), threshold, min_ttl);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&DataKey::Value(key))
    }

    pub fn bump(env: Env, key: u32) {
        let threshold = 1000;
        let min_ttl = 5000;
        if env.storage().persistent().has(&DataKey::Value(key)) {
            env.storage()
                .persistent()
                .extend_ttl(&DataKey::Value(key), threshold, min_ttl);
        }
    }
}
