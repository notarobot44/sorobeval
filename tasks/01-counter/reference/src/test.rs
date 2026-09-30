#![cfg(test)]
use super::*;
use soroban_sdk::Env;

fn client(env: &Env) -> CounterClient<'_> {
    let id = env.register(Counter, ());
    CounterClient::new(env, &id)
}

#[test]
fn starts_at_zero() {
    let env = Env::default();
    let c = client(&env);
    assert_eq!(c.get(), 0);
}

#[test]
fn increments_and_persists() {
    let env = Env::default();
    let c = client(&env);
    assert_eq!(c.increment(), 1);
    assert_eq!(c.increment(), 2);
    assert_eq!(c.get(), 2);
}

#[test]
fn get_does_not_mutate() {
    let env = Env::default();
    let c = client(&env);
    c.increment();
    assert_eq!(c.get(), 1);
    assert_eq!(c.get(), 1);
}
