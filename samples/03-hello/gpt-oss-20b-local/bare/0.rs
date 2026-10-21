#![no_std]

use soroban_sdk::{contractimpl, Env, Symbol, Vec};

#[derive(Default)]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut vec = Vec::new(&env);
        vec.push_back(Symbol::new("Hello"));
        vec.push_back(to);
        vec
    }
}
