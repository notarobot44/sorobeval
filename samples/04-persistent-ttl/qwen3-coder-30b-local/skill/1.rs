#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Bytes};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Value(u32),
}

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let data_key = DataKey::Value(key);
        env.storage().persistent().set(&data_key, &value);
        env.storage().persistent().extend_ttl(&data_key, 1000, 5000);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let data_key = DataKey::Value(key);
        env.storage().persistent().get(&data_key)
    }

    pub fn bump(env: Env, key: u32) {
        let data_key = DataKey::Value(key);
        if env.storage().persistent().has(&data_key) {
            env.storage().persistent().extend_ttl(&data_key, 1000, 5000);
        }
    }
}
