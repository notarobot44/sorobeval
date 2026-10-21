#![no_std]

use soroban_sdk::{
    contract, contractevent, contractimpl, Env,
    symbol_short, Symbol,
};

/// Storage keys used by the contract.
const KEY_REC: Symbol = symbol_short!("rec");
const KEY_CNT: Symbol = symbol_short!("cnt");

/// Event published on each record.  
/// The first topic is automatically the event name ("record") and the
/// second topic is `count`.  The data payload contains only `value`.
#[contractevent]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

/// Main contract.
#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    /// Store a new record, emit an event and return the total number of
    /// records so far (including this one).
    pub fn record(env: Env, value: u32) -> u32 {
        // Increment the counter.
        let mut count = env.storage().instance().get(&KEY_CNT).unwrap_or(0u32);
        count += 1;
        env.storage().instance().set(&KEY_CNT, &count);

        // Store the value.
        env.storage()
            .instance()
            .set(&KEY_REC, &value);

        // Emit the event.
        Record { count, value }.publish(&env);

        count
    }

    /// Return the most recently recorded value, if any.
    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&KEY_REC)
    }
}
