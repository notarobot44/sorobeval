#![no_std]

use soroban_sdk::{
    contract::{Contract, ContractClient},
    Env, Bytes, Symbol,
};

#[derive(Default)]
pub struct Persist;

impl Contract for Persist {
    type State = ();

    fn instantiate(env: &Env, _args: ()) -> Self::State {}
}

#[contract_interface]
impl Persist {
    /// Stores `value` under `key` in persistent storage and extends the entry's TTL
    /// so that it lives at least 5,000 ledgers from now (threshold 1,000).
    pub fn put(env: Env, key: u32, value: Bytes) -> () {
        const EXTEND_THRESHOLD: u64 = 1_000;
        const NEW_TTL: u64 = 5_000;

        let ledger_seq = env.ledger().sequence();
        // If the entry exists and its expiration is within threshold, extend it.
        if let Some(expiration) = env.storage().persistent().get_expiration(&key) {
            if expiration <= ledger_seq + EXTEND_THRESHOLD {
                env.storage()
                    .persistent()
                    .set_with_expiry(&key, &value, ledger_seq + NEW_TTL);
                return;
            }
        }

        // Otherwise set/overwrite the value with a new TTL.
        env.storage()
            .persistent()
            .set_with_expiry(&key, &value, ledger_seq + NEW_TTL);
    }

    /// Returns the value stored under `key` or None if it does not exist.
    pub fn fetch(env: Env, key: u32) -> Option<Bytes> {
        env.storage().persistent().get(&key)
    }

    /// Extends the TTL of an existing entry with the same parameters;
    /// does nothing if the key does not exist.
    pub fn bump(env: Env, key: u32) -> () {
        const NEW_TTL: u64 = 5_000;

        let ledger_seq = env.ledger().sequence();
        // Only extend if the entry exists
        if env.storage().persistent().contains_key(&key) {
            // Retrieve current value to re-set it with a new expiration.
            if let Some(value) = env.storage().persistent().get(&key) {
                env.storage()
                    .persistent()
                    .set_with_expiry(&key, &value, ledger_seq + NEW_TTL);
            }
        }
    }
}
