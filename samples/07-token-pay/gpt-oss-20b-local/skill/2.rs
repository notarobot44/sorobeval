#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address};

use soroban_sdk::token::Client as TokenClient;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
}

#[contract]
pub struct Payer;

#[contractimpl]
impl Payer {
    pub fn pay(env: Env, from: Address, to: Address, token: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Require the sender's authorization
        from.require_auth();

        // Perform the token transfer
        let client = TokenClient::new(&env, &token);
        client.transfer(&from, &to, &amount);

        Ok(())
    }
}
