#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Env, Symbol, BytesN, String, Vec, 
};

#[contract]
pub struct Recorder;

#[derive(Clone)]
#[contractevent]
struct Record {
    #[topic]
    name: Symbol,
    #[topic]
    count: u32,
    value: u32,
}

#[contractimpl]
impl Recorder {
    const VALUE_KEY: &'static str = "value";
    const COUNT_KEY: &'static str = "count";

    pub fn record(env: Env, value: u32) -> u32 {
        let storage = env.storage().instance();

        // Store the new value
        storage.set(&Symbol::short(Self::VALUE_KEY), &value);

        // Increment and store count
        let mut count: u32 = storage
            .get(&Symbol::short(Self::COUNT_KEY))
            .unwrap_or(0);
        count += 1;
        storage.set(&Symbol::short(Self::COUNT_KEY), &count);

        // Publish event
        Record {
            name: Symbol::short("record"),
            count,
            value,
        }
        .publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        let storage = env.storage().instance();
        storage.get(&Symbol::short(Self::VALUE_KEY))
    }
}
