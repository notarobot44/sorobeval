#![no_std]

use soroban_sdk::{contractimpl, contracttype, Env, Bytes};

#[contracttype]
pub enum DataKey {
    Value(u32),
}

pub struct Persist;

const MIN_TTL: u32 = 1000;
const TARGET_TTL: u32 = 5000;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage_key = DataKey::Value(key);
        env.storage().persistent().set(&storage_key, &value);

        // Extend TTL
        let current_ttl = env.storage().instance().ttl(&storage_key, MIN_TTL);
        if current_ttl < TARGET_TTL {
            env.storage().instance().extend_ttl(
                &storage_key,
                TARGET_TTL - current_ttl,
                MIN_TTL
            );
        }
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let storage_key = DataKey::Value(key);
        env.storage().persistent().get(&storage_key)
    }

    pub fn bump(env: Env, key: u32) {
        let storage_key = DataKey::Value(key);
        if env.storage().persistent().has(&storage_key) {
            // Extend TTL
            let current_ttl = env.storage().instance().ttl(&storage_key, MIN_TTL);
            if current_ttl < TARGET_TTL {
                env.storage().instance().extend_ttl(
                    &storage_key,
                    TARGET_TTL - current_ttl,
                    MIN_TTL
                );
            }
        }
    }
}
