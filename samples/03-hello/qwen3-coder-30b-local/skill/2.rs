#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut vec = Vec::new(&env);
        vec.push(symbol_short!("Hello"));
        vec.push(to);
        vec
    }
}
