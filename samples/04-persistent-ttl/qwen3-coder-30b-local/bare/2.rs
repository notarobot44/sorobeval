#![no_std]
use soroban_sdk::{contract, contractimpl, env::Env, Bytes, Vec};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let storage = env.storage().persistent();
        storage.set(&key, &value);
        
        // Extend TTL to at least 5000 ledgers from now
        let current_ledger = env.ledger().sequence();
        let ttl_threshold = 1000;
        let ttl_extension = 5000;
        
        // Check if the entry exists and extend TTL if needed
        if let Some(_existing) = storage.get::<u32, Bytes>(&key) {
            // The entry already exists, extend its TTL
            storage.extend_ttl(&key, ttl_threshold, ttl_extension);
        } else {
            // Entry doesn't exist, set it with TTL
            storage.set_ttl(&key, ttl_threshold, ttl_extension);
        }
    }
    
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let storage = env.storage().persistent();
        storage.get::<u32, Bytes>(&key)
    }
    
    pub fn bump(env: Env, key: u32) {
        let storage = env.storage().persistent();
        
        // Check if the entry exists before extending TTL
        if let Some(_value) = storage.get::<u32, Bytes>(&key) {
            // Entry exists, extend its TTL
            let ttl_threshold = 1000;
            let ttl_extension = 5000;
            storage.extend_ttl(&key, ttl_threshold, ttl_extension);
        }
        // If entry doesn't exist, do nothing (as per requirements)
    }
}
