#![no_std]

use soroban_sdk::{contracterror, token::Client as TokenClient, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
}

pub struct Payer;

pub fn pay(env: Env, from: Address, to: Address, token: Address, amount: i128) -> Result<(), Error> {
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }

    let client = TokenClient::new(&env, &token);
    client.transfer(&from, &to, &amount);

    Ok(())
}
