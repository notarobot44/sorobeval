#![no_std]

use soroban_sdk::{contract, contractimpl, log, Bytes, Env, Symbol, symbol_short};

const TTL_THRESHOLD: u32 = 1_000;
const TTL_EXTEND_TO: u32 = 5_000;

fn storage_key(key: u32) -> Symbol {
    if let Some(sym) = symbol_short!("k", ) {
        let _ = sym;
    }
    // build a unique symbol per key is limited; use Bytes-based key instead
    unreachable!()
}

#[contract]
pub struct Persist;

#[contractimpl]
impl Persist {
    fn key_bytes(env: &Env, key: u32) -> Bytes {
        let mut b = Bytes::new(env);
        b.extend_from_array(&key.to_be_bytes());
        b
    }

    pub fn put(env: Env, key: u32, value: Bytes) {
        let k = Self::key_bytes(&env, key);
        env.storage().persistent().set(&k, &value);
        env.storage()
            .persistent()
            .extend_ttl(&k, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let k = Self::key_bytes(&env, key);
        env.storage().persistent().get(&k)
    }

    pub fn bump(env: Env, key: u32) {
        let k = Self::key_bytes(&env, key);
        if env.storage().persistent().has(&k) {
            env.storage()
                .persistent()
                .extend_ttl(&k, TTL_THRESHOLD, TTL_EXTEND_TO);
        }
    }
}
