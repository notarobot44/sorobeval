#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, GatedClient<'static>, Address) {
    let env = Env::default();
    let id = env.register(Gated, ());
    let c = GatedClient::new(&env, &id);
    let admin = Address::generate(&env);
    (env, c, admin)
}

#[test]
fn admin_can_set() {
    let (env, c, admin) = setup();
    env.mock_all_auths();
    c.init(&admin);
    c.set_value(&9);
    assert_eq!(c.value(), 9);
}

#[test]
fn init_twice_fails() {
    let (env, c, admin) = setup();
    env.mock_all_auths();
    c.init(&admin);
    assert_eq!(c.try_init(&admin), Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn set_before_init_fails() {
    let (env, c, _admin) = setup();
    env.mock_all_auths();
    assert_eq!(c.try_set_value(&1), Err(Ok(Error::NotInitialized)));
    assert_eq!(c.value(), 0);
}

#[test]
fn unauthorized_set_is_rejected() {
    let (env, c, admin) = setup();
    env.mock_all_auths();
    c.init(&admin);
    // No auths mocked from here: require_auth must fail.
    env.set_auths(&[]);
    assert!(c.try_set_value(&5).is_err());
    assert_eq!(c.value(), 0);
}
