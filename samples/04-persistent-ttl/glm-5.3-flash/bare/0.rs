#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env, Symbol, symbol_short};

const LEDGER_THRESHOLD: u32 = 1_000;
const LEDGER_EXTEND: u32 = 5_000;

#[contract]
pub struct Persist;

fn key_sym(key: u32) -> Symbol {
    symbol_short!(&key.to_string()[..])
}

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let k = key_sym(key);
        env.storage().persistent().set(&k, &value);
        env.storage()
            .persistent()
            .extend_ttl(&k, LEDGER_THRESHOLD, LEDGER_EXTEND);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let k = key_sym(key);
        env.storage().persistent().get(&k)
    }

    pub fn bump(env: Env, key: u32) {
        let k = key_sym(key);
        if env.storage().persistent().has(&k) {
            env.storage()
                .persistent()
                .extend_ttl(&k, LEDGER_THRESHOLD, LEDGER_EXTEND);
        }
    }
}
