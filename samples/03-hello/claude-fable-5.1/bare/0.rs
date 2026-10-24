#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut v: Vec<Symbol> = Vec::new(&env);
        v.push_back(symbol_short!("Hello"));
        v.push_back(to);
        v
    }
}
