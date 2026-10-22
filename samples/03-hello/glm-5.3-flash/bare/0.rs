#![no_std]

use soroban_sdk::{contract, contractimpl, vec, Env, IntoVal, Symbol, Vec};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![
            &env,
            Symbol::new(&env, "Hello"),
            to,
        ]
    }
}
