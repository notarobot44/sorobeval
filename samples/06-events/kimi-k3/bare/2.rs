#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env};

#[contractevent(name = "record")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    #[topic]
    pub count: u32,
    pub value: u32,
}

const VALUE: soroban_sdk::Symbol = symbol_short!("VALUE");
const COUNT: soroban_sdk::Symbol = symbol_short!("COUNT");

#[contract]
pub struct Recorder;

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNT).unwrap_or(0) + 1;
        env.storage().instance().set(&VALUE, &value);
        env.storage().instance().set(&COUNT, &count);

        Record { count, value }.publish(&env);

        count
    }

    pub fn last(env: Env) -> Option<u32> {
        env.storage().instance().get(&VALUE)
    }
}
