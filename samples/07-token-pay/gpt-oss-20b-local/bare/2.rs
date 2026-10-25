#![no_std]

use soroban_sdk::{
    contracterror,
    token::Client,
    Address, Env, Symbol,
};

#[derive(Clone)]
pub struct Payer;

#[contracterror]
#[repr(u32)]
pub enum Error {
    #[default]
    InvalidAmount = 1,
}

impl Payer {
    pub fn pay(
        env: Env,
        from: Address,
        to: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Require authorization from the `from` account
        env.require_auth(from.clone());

        // Create a client for the token contract and transfer tokens
        let client = Client::new(&env, &token);
        client.transfer(&from, &to, &amount);

        Ok(())
    }
}
