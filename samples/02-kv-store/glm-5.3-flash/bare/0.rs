#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct Store;

const KEY: Symbol = symbol_short!("v");

#[contractimpl]
impl Store {
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage().instance().set(&key, &value);
    }

    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage().instance().get(&key)
    }

    pub fn remove(env: Env, key: Symbol) {
        env.storage().instance().remove(&key);
    }
}
