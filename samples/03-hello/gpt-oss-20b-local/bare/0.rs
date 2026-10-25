#![no_std]

extern crate soroban_sdk;

use soroban_sdk::{Env, Symbol, Vec};

pub struct Hello;

impl Hello {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        let mut v = Vec::new(&env);
        v.push_back(Symbol::native("Hello"));
        v.push_back(to);
        v
    }
}
