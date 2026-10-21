#![no_std]

use soroban_sdk::{contractevent, contractimpl, Env, Symbol};

#[contractevent]
#[derive(Clone, Debug)]
pub struct Record {
    pub count: u32,
    pub value: u32,
}

pub struct Recorder;

#[contractimpl]
impl Recorder {
    pub fn record(env: Env, value: u32) -> u32 {
        let key = Symbol::new(&env, "rec");
        let count = env
            .storage()
            .instance()
            .get::<Symbol, u32>(&key)
            .unwrap_or(0);
        let new_count = count + 1;
        env.storage().instance().set(&key, &new_count);

        Record {
            count: new_count,
            value,
        }
        .publish(&env);

        new_count
    }

    pub fn last(env: Env) -> Option<u32> {
        let key = Symbol::new(&env, "rec");
        env.storage().instance().get::<Symbol, u32>(&key)
    }
}
