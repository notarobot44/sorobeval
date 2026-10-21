#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        // Create a new vector using the environment.
        let mut vec = Vec::<Symbol>::new(&env);
        // Push the symbol "Hello" and the supplied `to` value.
        vec.push_back(Symbol::new("Hello"));
        vec.push_back(to);
        vec
    }
}
