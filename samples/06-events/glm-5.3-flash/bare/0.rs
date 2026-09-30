#![no_std]

use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env, Symbol};

/// Instance-storage key holding the total number of records so far.
const COUNT: Symbol = symbol_short!("count");

/// Instance-storage key holding the most recently recorded value.
const LAST: Symbol = symbol_short!("last");

/// Event emitted on every successful `record` call.
///
/// Topics: the event name `record` (derived from the struct name) followed by
/// the running record `count`. Data: the recorded `value`.
#[contractevent]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    /// Stores `value` in instance storage, publishes a `Record` event, and
    /// returns the number of records so far (1 for the first call).
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNT).unwrap_or(0) + 1;

        env.storage().instance().set(&COUNT, &count);
        env.storage().instance().set(&LAST, &value);

        Record { count, value }.publish(&env);

        count
    }

    /// Returns the most recently recorded value, if any. Publishes no events.
    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&LAST)
    }
}
