#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut result = Vec::new(&env);
        result.push_back(&Symbol::new(&env, "Hello"));
        result.push_back(&to);
        result
    }
}
