#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Counter;

const COUNTER: Symbol = Symbol::short("COUNTER");

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let mut counter = env.storage().instance().get::<u32>(COUNTER).unwrap_or(0);
        counter += 1;
        env.storage().instance().set_ttl(1000); // Extend TTL
        env.storage().instance().set(COUNTER, counter);
        counter
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get::<u32>(COUNTER).unwrap_or(0)
    }
}
