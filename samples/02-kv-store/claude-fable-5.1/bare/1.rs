#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Store;

#[contractimpl]
impl Store {
    /// Stores `value` under `key` in instance storage.
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage().instance().set(&key, &value);
    }

    /// Returns the value stored under `key`, or `None` if absent.
    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage().instance().get::<Symbol, i128>(&key)
    }

    /// Removes `key` from instance storage.
    pub fn remove(env: Env, key: Symbol) {
        env.storage().instance().remove(&key);
    }
}
