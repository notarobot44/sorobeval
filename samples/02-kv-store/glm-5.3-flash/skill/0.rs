#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Store;

#[contractimpl]
impl Store {
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage().instance().set(&key, &value);
        env.storage().instance().extend_ttl(30 * 17280, 120 * 17280);
    }

    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage().instance().get(&key)
    }

    pub fn remove(env: Env, key: Symbol) {
        env.storage().instance().remove(&key);
    }
}
