#![no_std]
use soroban_sdk::{contract, contractimpl, env::Env, Bytes, StorageType};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage_key = key.to_be_bytes();
        env.storage().persistent().set(&storage_key, &value);
        
        // Extend TTL to at least 5000 ledgers from now
        let current_ledger = env.ledger().sequence();
        let ttl = current_ledger + 5000;
        env.storage().persistent().extend_ttl(&storage_key, 1000, 5000);
    }
    
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let storage_key = key.to_be_bytes();
        env.storage().persistent().get(&storage_key)
    }
    
    pub fn bump(env: Env, key: u32) {
        let storage_key = key.to_be_bytes();
        if env.storage().persistent().has(&storage_key) {
            let current_ledger = env.ledger().sequence();
            let ttl = current_ledger + 5000;
            env.storage().persistent().extend_ttl(&storage_key, 1000, 5000);
        }
    }
}
