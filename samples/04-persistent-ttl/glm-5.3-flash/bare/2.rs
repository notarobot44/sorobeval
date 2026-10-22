#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env, Symbol, Vec};

const TTL_THRESHOLD: u32 = 1_000;
const TTL_EXTEND_TO: u32 = 5_000;

#[contract]
pub struct Persist;

fn storage_key(key: u32) -> Symbol {
    Symbol::new(&Env::default(), "v")
}

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let sk = storage_key(key);
        env.storage().persistent().set(&sk, &value);
        env.storage()
            .persistent()
            .extend_ttl(&sk, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let sk = storage_key(key);
        env.storage().persistent().get(&sk)
    }

    pub fn bump(env: Env, key: u32) {
        let sk = storage_key(key);
        if env.storage().persistent().has(&sk) {
            env.storage()
                .persistent()
                .extend_ttl(&sk, TTL_THRESHOLD, TTL_EXTEND_TO);
        }
    }
}
