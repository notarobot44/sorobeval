#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const KEY: Symbol = symbol_short!("COUNT");

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let mut value: u32 = env.storage().instance().get(&KEY).unwrap_or(0);
        value += 1;
        env.storage().instance().set(&KEY, &value);
        env.storage()
            .instance()
            .extend_ttl(50, 100);
        value
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&KEY).unwrap_or(0)
    }
}
