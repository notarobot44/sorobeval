#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct Persist;

const EXTEND_TO: u32 = 5_000;
const THRESHOLD: u32 = 1_000;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let k = key;
        env.storage().persistent().set(&k, &value);
        env.storage()
            .persistent()
            .extend_ttl(&k, THRESHOLD, EXTEND_TO);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    pub fn bump(env: Env, key: u32) {
        if env.storage().persistent().has(&key) {
            env.storage()
                .persistent()
                .extend_ttl(&key, THRESHOLD, EXTEND_TO);
        }
    }
}
