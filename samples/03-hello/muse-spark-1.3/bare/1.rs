#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut v = Vec::new(&env);
        v.push_back(Symbol::new(&env, "Hello"));
        v.push_back(to);
        v
    }
}
