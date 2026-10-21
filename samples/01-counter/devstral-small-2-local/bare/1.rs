#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Counter;

const KEY: Symbol = Symbol::new(&Env::new(), "counter");

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let mut counter: u32 = env.storage().instance().get(&KEY).unwrap_or(0);
        counter += 1;
        env.storage().instance().set(&KEY, &counter);
        env.storage().instance().extend_ttl(&KEY, 60 * 60); // Extend TTL by 1 hour
        counter
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&KEY).unwrap_or(0)
    }
}
