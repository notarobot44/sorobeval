#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    /// Returns a two‑element vector: the symbol `"Hello"` followed by `to`.
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        // Start with an empty vector using the environment.
        let mut result: Vec<Symbol> = vec![&env];
        // Push the literal "Hello" and the supplied symbol.
        result.push(Symbol::new(&env, "Hello"));
        result.push(to);
        result
    }
}
