#![no_std]
use soroban_sdk::{contract, env::Env, symbol, vec, Vec, Symbol};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}
