#![no_std]

use soroban_sdk::{Env, Symbol, Vec};

#[soroban_sdk::contract]
pub struct Hello;

#[soroban_sdk::contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut vec = Vec::<Symbol>::new(&env);
        vec.push_back(Symbol::from_str("Hello"));
        vec.push_back(to);
        vec
    }
}
