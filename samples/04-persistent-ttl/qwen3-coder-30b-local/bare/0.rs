#![no_std]
use soroban_sdk::{contract, contractimpl, env::Env, Bytes, Storage, StorageType};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = Storage::new(&env);
        let key_bytes = key.to_le_bytes();
        storage.set(&key_bytes, &value);
        Self::bump_internal(&storage, &key_bytes);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let storage = Storage::new(&env);
        let key_bytes = key.to_le_bytes();
        storage.get(&key_bytes)
    }

    pub fn bump(env: Env, key: u32) {
        let storage = Storage::new(&env);
        let key_bytes = key.to_le_bytes();
        Self::bump_internal(&storage, &key_bytes);
    }

    fn bump_internal(storage: &Storage, key: &[u8]) {
        if storage.has(key) {
            storage.extend_ttl(key, 1000, 5000);
        }
    }
}
