#![no_std]

use soroban_sdk::{
    contract::{contractimpl, contracterror},
    Env,
    Address,
    token::Client as TokenClient,
};

#[contracterror]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
}

pub struct Payer;

#[contractimpl]
impl Payer {
    pub fn pay(env: Env, from: Address, to: Address, token: Address, amount: i128) -> Result<(), Error> {
        // Require authorization of the `from` address
        env.invocation().require_auth(&from);

        // Validate amount
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Perform the token transfer via the token contract client
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&from, &to, amount)?;

        Ok(())
    }
}
