#![no_std]
use soroban_sdk::{contract, env::Env, symbol, ContractError, Symbol, StorageType};

#[contract]
pub struct Store;

#[contract]
impl Store {
    pub fn set(env: Env, key: Symbol, value: i128) -> Result<(), ContractError> {
        env.storage().set(&key, &value);
        Ok(())
    }

    pub fn get(env: Env, key: Symbol) -> Option<i128> {
        env.storage().get(&key)
    }

    pub fn remove(env: Env, key: Symbol) -> Result<(), ContractError> {
        env.storage().remove(&key);
        Ok(())
    }
}
