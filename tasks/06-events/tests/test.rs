#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Events as _, Address, Env, Event, Val, Vec};

#[test]
fn record_publishes_exactly_one_event_per_call() {
    let env = Env::default();
    let id = env.register(Recorder, ());
    let c = RecorderClient::new(&env, &id);

    assert_eq!(c.record(&7), 1);
    assert_eq!(env.events().all(), [Record { count: 1, value: 7 }.to_xdr(&env, &id)]);

    assert_eq!(c.record(&8), 2);
    assert_eq!(env.events().all(), [Record { count: 2, value: 8 }.to_xdr(&env, &id)]);
    assert_eq!(c.last(), Some(8));
}

#[test]
fn last_publishes_nothing() {
    let env = Env::default();
    let id = env.register(Recorder, ());
    let c = RecorderClient::new(&env, &id);
    c.record(&1);
    assert_eq!(c.last(), Some(1));
    let none: Vec<(Address, Vec<Val>, Val)> = Vec::new(&env);
    assert_eq!(env.events().all(), none);
}
