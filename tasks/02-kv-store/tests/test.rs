#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, Env};

fn client(env: &Env) -> StoreClient<'_> {
    let id = env.register(Store, ());
    StoreClient::new(env, &id)
}

#[test]
fn missing_key_is_none() {
    let env = Env::default();
    let c = client(&env);
    assert_eq!(c.get(&symbol_short!("nope")), None);
}

#[test]
fn set_then_get() {
    let env = Env::default();
    let c = client(&env);
    c.set(&symbol_short!("a"), &42);
    c.set(&symbol_short!("b"), &-7);
    assert_eq!(c.get(&symbol_short!("a")), Some(42));
    assert_eq!(c.get(&symbol_short!("b")), Some(-7));
}

#[test]
fn overwrite_and_remove() {
    let env = Env::default();
    let c = client(&env);
    c.set(&symbol_short!("a"), &1);
    c.set(&symbol_short!("a"), &2);
    assert_eq!(c.get(&symbol_short!("a")), Some(2));
    c.remove(&symbol_short!("a"));
    assert_eq!(c.get(&symbol_short!("a")), None);
}
