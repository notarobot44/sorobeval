#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Bytes, Env, Symbol};

const LEDGER_THRESHOLD: u32 = 1_000;
const LEDGER_EXTEND: u32 = 5_000;

#[contract]
pub struct Persist;

const KEY_PREFIX: Symbol = symbol_short!("persist");

fn storage_key(env: &Env, key: u32) -> (Symbol, u32) {
    (KEY_PREFIX, key)
}

#[contractimpl]
impl Persist {
    pub fn put(env: Env, key: u32, value: Bytes) {
        let sk = storage_key(&env, key);
        let storage = env.storage().persistent();
        storage.set(&sk, &value);
        storage.extend_ttl(&sk, LEDGER_THRESHOLD, LEDGER_EXTEND);
    }

    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        let sk = storage_key(&env, key);
        env.storage().persistent().get(&sk)
    }

    pub fn bump(env: Env, key: u32) {
        let sk = storage_key(&env, key);
        let storage = env.storage().persistent();
        if storage.has(&sk) {
            storage.extend_ttl(&sk, LEDGER_THRESHOLD, LEDGER_EXTEND);
        }
    }
}
