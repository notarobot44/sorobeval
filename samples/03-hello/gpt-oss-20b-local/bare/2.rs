#![no_std]

use soroban_sdk::{contract, Env, Symbol, Vec};

#[derive(Default)]
pub struct Hello;

#[contract]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut v = Vec::new(&env);
        v.push_back(Symbol::new("Hello"));
        v.push_back(to);
        v
    }
}
