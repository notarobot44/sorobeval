#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut res = Vec::new(&env);
        res.push_back(&Symbol::from_str(&env, "Hello"));
        res.push_back(&to);
        res
    }
}
