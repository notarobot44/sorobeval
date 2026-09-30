#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::{storage::Persistent as _, Ledger}, Address, Bytes, Env};

fn setup() -> (Env, Address) {
    let env = Env::default();
    env.ledger().with_mut(|l| {
        l.sequence_number = 100;
        l.min_persistent_entry_ttl = 100;
        l.max_entry_ttl = 100_000;
    });
    let id = env.register(Persist, ());
    (env, id)
}

#[test]
fn put_and_fetch() {
    let (env, id) = setup();
    let c = PersistClient::new(&env, &id);
    let v = Bytes::from_slice(&env, b"hello");
    c.put(&7, &v);
    assert_eq!(c.fetch(&7), Some(v));
    assert_eq!(c.fetch(&8), None);
}

#[test]
fn put_extends_ttl_to_at_least_5000() {
    let (env, id) = setup();
    let c = PersistClient::new(&env, &id);
    c.put(&1, &Bytes::from_slice(&env, b"x"));
    let ttl = env.as_contract(&id, || env.storage().persistent().get_ttl(&1u32));
    assert!(ttl >= 5_000, "ttl was {ttl}");
}

#[test]
fn bump_missing_key_is_noop() {
    let (env, id) = setup();
    let c = PersistClient::new(&env, &id);
    c.bump(&99); // must not panic
    assert_eq!(c.fetch(&99), None);
}
