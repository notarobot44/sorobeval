#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut greeting = Vec::new(&env);
        greeting.push_back(symbol_short!("Hello"));
        greeting.push_back(to);
        greeting
    }
}
