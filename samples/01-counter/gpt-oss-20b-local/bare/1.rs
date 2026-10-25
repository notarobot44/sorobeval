#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Storage};

#[contract]
pub struct Counter;

const COUNTER_KEY: &str = "cnt";

fn counter_key() -> Symbol {
    Symbol::short(COUNTER_KEY)
}

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let key = counter_key();

        // Read current value or default to 0
        let current_val = env.storage().instance().get(&key);
        let current = match current_val {
            Some(v) => v.into_val::<u32>(&env),
            None => 0,
        };

        let new_val = current + 1;

        // Store the updated value
        env.storage()
            .instance()
            .set(&key, &Val::from_u32(&env, new_val));

        // Extend TTL for this key (e.g., to 1000 units)
        env.storage().instance().set_ttl(key.clone(), 1_000);

        new_val
    }

    pub fn get(env: Env) -> u32 {
        let key = counter_key();

        match env.storage().instance().get(&key) {
            Some(v) => v.into_val::<u32>(&env),
            None => 0,
        }
    }
}
