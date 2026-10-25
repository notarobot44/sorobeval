#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Bytes};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Value(u32),
}

#[contract]
pub struct Persist;

const BUMP_THRESHOLD: u32 = 1000;
const BUMP_TO: u32 = 5000;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        env.storage().persistent().set(&DataKey::Value(key), &value);
        env.storage().persistent().extend_ttl(&DataKey::Value(key), BUMP_THRESHOLD, BUMP_TO);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&DataKey::Value(key))
    }

    pub fn bump(env: Env, key: u32) {
        if env.storage().persistent().has(&DataKey::Value(key)) {
            env.storage().persistent().extend_ttl(&DataKey::Value(key), BUMP_THRESHOLD, BUMP_TO);
        }
    }
}
