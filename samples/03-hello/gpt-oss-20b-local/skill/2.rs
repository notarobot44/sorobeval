#![no_std]

use soroban_sdk::{contract, Env, Symbol, Vec};

#[contract]
pub struct Hello;

#[soroban_sdk::contractimpl]
impl Hello {
    /// Returns a vector containing the symbols `Hello` and the provided `to`.
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let hello_sym = Symbol::new(&env, "Hello");
        let mut res = Vec::<Symbol>::new(&env);
        res.push_back(hello_sym);
        res.push_back(to);
        res
    }
}
