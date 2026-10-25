#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, vec, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
}

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}
