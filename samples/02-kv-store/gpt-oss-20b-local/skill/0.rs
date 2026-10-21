#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Store;

#[contractimpl]
impl Store {
    /// Stores the given value under the supplied key.
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage().instance().set(&key, &value);
    }

    /// Retrieves the value stored at `key`, or `None` if it has never been set.
    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage().instance().get(&key)
    }

    /// Removes the entry for `key`. Subsequent calls to `get` will return `None`.
    pub fn remove(env: Env, key: Symbol) {
        env.storage().instance().remove(&key);
    }
}
