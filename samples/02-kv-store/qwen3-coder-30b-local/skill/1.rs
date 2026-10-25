#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Value(Symbol),
}

#[contract]
pub struct Store;

#[contractimpl]
impl Store {
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage().instance().set(&DataKey::Value(key), &value);
    }

    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage().instance().get(&DataKey::Value(key))
    }

    pub fn remove(env: Env, key: Symbol) {
        env.storage().instance().remove(&DataKey::Value(key));
    }
}
