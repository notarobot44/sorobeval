#![no_std]

use soroban_sdk::{contractimpl, contracttype, Env, Bytes};

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
        let key = DataKey::Value(key);
        env.storage().persistent().set(&key, &value);
        env.storage().persistent().extend_ttl(
            &key,
            threshold,
            min_ttl,
        );
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let key = DataKey::Value(key);
        env.storage().persistent().get(&key)
    }

    pub fn bump(env: Env, key: u32) {
        let threshold = 1000;
        let min_ttl = 5000;
        let key = DataKey::Value(key);
        if env.storage().persistent().has(&key) {
            env.storage().persistent().extend_ttl(
                &key,
                threshold,
                min_ttl,
            );
        }
    }
}
