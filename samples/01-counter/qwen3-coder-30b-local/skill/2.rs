#![no_std]
use soroban_sdk::{contract, contractimpl, Env, symbol_short};

#[contract]
pub struct Counter;

const COUNTER: symbol_short! = symbol_short!("cnt");

#[contractimpl]
impl Counter {
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        
        // Extend TTL so contract state is not archived
        env.storage().instance().extend_ttl(120 * 17280, 180 * 17280);
        
        count
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}
