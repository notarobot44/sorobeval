#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env, Symbol};

const LAST: Symbol = symbol_short!("LAST");
const COUNT: Symbol = symbol_short!("COUNT");

/// Emitted once per `record` call.
/// Topics: ("record", count); data: { value }.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    /// Store `value`, emit a `Record` event, and return the number of records so far.
    pub fn record(env: Env, value: u32) -> u32 {
        let storage = env.storage().instance();

        let count: u32 = storage.get(&COUNT).unwrap_or(0);
        let count = count.checked_add(1).expect("record count overflow");

        storage.set(&LAST, &value);
        storage.set(&COUNT, &count);

        Record { count, value }.publish(&env);

        count
    }

    /// The most recently recorded value, if any.
    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&LAST)
    }
}
