#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env};

/// Emitted on every `record` call.
/// Topics: ("record", count) — the event name topic defaults to the
/// snake-cased struct name, and `count` is marked as a topic.
/// Data: value.
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
    /// Stores `value` in instance storage, publishes a `Record` event,
    /// and returns the number of records so far (1 for the first call).
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env
            .storage()
            .instance()
            .get(&symbol_short!("count"))
            .unwrap_or(0)
            + 1;

        env.storage().instance().set(&symbol_short!("count"), &count);
        env.storage().instance().set(&symbol_short!("last"), &value);

        Record { count, value }.publish(&env);

        count
    }

    /// The most recently recorded value, if any. Emits no events.
    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&symbol_short!("last"))
    }
}
