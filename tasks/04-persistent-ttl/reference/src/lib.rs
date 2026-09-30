#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

const THRESHOLD: u32 = 1_000;
const EXTEND_TO: u32 = 5_000;

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let s = env.storage().persistent();
        s.set(&key, &value);
        s.extend_ttl(&key, THRESHOLD, EXTEND_TO);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    pub fn bump(env: Env, key: u32) {
        let s = env.storage().persistent();
        if s.has(&key) {
            s.extend_ttl(&key, THRESHOLD, EXTEND_TO);
        }
    }
}

#[cfg(test)]
mod test;
