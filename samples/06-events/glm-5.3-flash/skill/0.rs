#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env};

/// Emitted once per successful `record` call.
/// Topics: ("record", count) — data payload: { value }.
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
    /// and returns the total number of records so far (1 for the first call).
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env
            .storage()
            .instance()
            .get(&symbol_short!("count"))
            .unwrap_or(0)
            + 1;

        env.storage().instance().set(&symbol_short!("last"), &value);
        env.storage().instance().set(&symbol_short!("count"), &count);

        Record { count, value }.publish(&env);

        count
    }

    /// The most recently recorded value, or `None` if nothing has been recorded.
    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&symbol_short!("last"))
    }
}
