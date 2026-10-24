#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env, Symbol, symbol_short};

const TTL_THRESHOLD: u32 = 1_000;
const TTL_EXTEND_TO: u32 = 5_000;

fn storage_key(key: u32) -> Symbol {
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&key.to_be_bytes());
    symbol_short!(&core::str::from_utf8(&buf).unwrap_or("k"))
}

#[contract]
pub struct Persist;

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
