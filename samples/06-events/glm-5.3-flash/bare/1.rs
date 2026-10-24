#![no_std]

use soroban_sdk::{contract, contracttype, env::Env, symbol_short, Address, String, Vec};

#[contract]
pub struct Recorder;

#[contracttype]
pub struct RecordEvent {
    pub count: u32,
    pub value: u32,
}

const KEY: soroban_sdk::Symbol = symbol_short!("record");

#[contractevent(name = "record")]
struct Record {
    #[topic]
    count: u32,
    value: u32,
}

pub fn record(env: Env, value: u32) -> u32;
pub fn last(env: Env) -> Option<u32>;
