#![no_std]

use soroban_sdk::{contractimpl, Env, Symbol};

pub struct Store;

#[contractimpl]
impl Store {
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage()
            .instance()
            .set(&key, &value);
    }

    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage()
            .instance()
            .get::<Symbol, i128>(&key)
    }

    pub fn remove(env: Env, key: Symbol) {
        env.storage()
            .instance()
            .remove(&key);
    }
}
