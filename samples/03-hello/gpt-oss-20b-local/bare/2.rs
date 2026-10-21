#![no_std]

use soroban_sdk::{Env, Symbol, Vec};

pub struct Hello;

impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut v = Vec::<Symbol>::new(&env);
        v.push_back(Symbol::from_str("Hello").unwrap());
        v.push_back(to);
        v
    }
}
