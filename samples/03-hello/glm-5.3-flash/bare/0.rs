#![no_std]

use soroban_sdk::{symbol_short, Env, Symbol, Vec};

#[contracttype]
pub struct Hello;

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut v = Vec::new(&env);
        v.push_back(symbol_short!("Hello"));
        v.push_back(to);
        v
    }
}
