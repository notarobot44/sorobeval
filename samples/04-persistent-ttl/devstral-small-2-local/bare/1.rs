#![no_std]

use soroban_sdk::{contractimpl, contracttype, Bytes, Env};

#[contracttype]
pub enum DataKey {
    Value(u32),
}

pub struct Persist;

const MIN_TTL: u32 = 1000;
const DESIRED_TTL: u32 = 5000;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let key = DataKey::Value(key);
        env.storage().persistent().set(&key, &value);
        Self::bump_ttl(&env, &key);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let key = DataKey::Value(key);
        env.storage().persistent().get::<_, Bytes>(&key)
    }

    pub fn bump(env: Env, key: u32) {
        let key = DataKey::Value(key);
        if env.storage().persistent().has(&key) {
            Self::bump_ttl(&env, &key);
        }
    }

    fn bump_ttl(env: &Env, key: &DataKey) {
        let current_ttl = env.storage().persistent().ttl(key).unwrap_or(0);
        if current_ttl < MIN_TTL {
            env.storage()
                .persistent()
                .extend_ttl(key, DESIRED_TTL - current_ttl);
        }
    }
}
