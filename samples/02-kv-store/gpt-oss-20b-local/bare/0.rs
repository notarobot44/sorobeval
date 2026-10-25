#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Store;

#[contractimpl]
impl Store {
    /// Stores the value under the given key.
    pub fn set(env: Env, key: Symbol, value: i128) {
        let storage = env.storage().instance();
        // SAFETY: `Symbol` implements `IntoVal`, and `i128` is supported by the SDK.
        storage.set(&key, &value);
    }

    /// Retrieves the value stored under the given key, if any.
    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        let storage = env.storage().instance();
        match storage.get::<i128>(&key) {
            Ok(opt) => opt,
            Err(_) => None,
        }
    }

    /// Removes the value stored under the given key.
    pub fn remove(env: Env, key: Symbol) {
        let storage = env.storage().instance();
        // SAFETY: `Symbol` implements `IntoVal`.
        storage.remove(&key);
    }
}
