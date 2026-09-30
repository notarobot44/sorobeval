#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        env.storage().persistent().set(&key, &value);
        env.storage().persistent().extend_ttl(&key, 1000, 5000);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    pub fn bump(env: Env, key: u32) {
        if env.storage().persistent().has(&key) {
            env.storage().persistent().extend_ttl(&key, 1000, 5000);
        }
    }
}
