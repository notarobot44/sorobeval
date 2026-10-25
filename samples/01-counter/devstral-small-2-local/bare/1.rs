#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Counter;

const KEY: &str = "counter";

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let mut counter: u32 = env
            .storage()
            .instance()
            .get(&env, &Symbol::new(&env, KEY))
            .unwrap_or(0);
        counter += 1;
        env.storage().instance().set(&env, &Symbol::new(&env, KEY), &counter);
        env.storage().instance().extend_ttl(
            &env,
            &env.storage().instance().time_to_live(&env).unwrap(),
            31536000, // 1 year in seconds
        );
        counter
    }

    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&env, &Symbol::new(&env, KEY))
            .unwrap_or(0)
    }
}
