#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn greets() {
    let env = Env::default();
    let id = env.register(Hello, ());
    let c = HelloClient::new(&env, &id);
    let out = c.hello(&symbol_short!("Dev"));
    assert_eq!(out, vec![&env, symbol_short!("Hello"), symbol_short!("Dev")]);
    assert_eq!(out.len(), 2);
}
