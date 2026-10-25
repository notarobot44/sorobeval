#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let key = Symbol::new(&env, "counter");
        let counter: u32 = env.storage().instance().get(&key).unwrap_or(0);
        let new_counter = counter + 1;
        env.storage().instance().set(&key, &new_counter);
        env.storage().instance().extend_ttl(
            &key,
            3600 * 24, // Extend TTL by 1 day (in ledgers)
        );
        new_counter
    }

    pub fn get(env: Env) -> u32 {
        let key = Symbol::new(&env, "counter");
        env.storage().instance().get(&key).unwrap_or(0)
    }
}
