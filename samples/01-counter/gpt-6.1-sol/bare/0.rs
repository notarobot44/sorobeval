#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let key = symbol_short!("COUNTER");
        let current: u32 = env.storage().instance().get(&key).unwrap_or(0);
        let next = current.checked_add(1).expect("counter overflow");

        env.storage().instance().set(&key, &next);
        env.storage().instance().extend_ttl(50, 100);

        next
    }

    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("COUNTER"))
            .unwrap_or(0)
    }
}
