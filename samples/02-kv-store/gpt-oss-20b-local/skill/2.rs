#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Store;

#[contractimpl]
impl Store {
    /// Stores the given `value` under the provided `key`.
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage().instance().set(&key, &value);
    }

    /// Returns the stored value for `key`, or `None` if it was never set.
    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage()
            .instance()
            .get::<i128>(&key)
    }

    /// Deletes the entry associated with `key`. Subsequent `get` calls will return `None`.
    pub fn remove(env: Env, key: Symbol) {
        env.storage().instance().remove(&key);
    }
}
