#![no_std]

use soroban_sdk::{
    contract, contractimpl,
    env::Env,
    symbol_short,
    vec, Vec,
    Symbol,
};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        // Build a vector with the environment handle
        let mut v = vec![&env, Symbol::new(&env, "Hello")];
        v.push_back(to);
        v
    }
}
