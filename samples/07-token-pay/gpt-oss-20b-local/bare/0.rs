#![no_std]

use soroban_sdk::{contractimpl, contracttype, Env, Address};

#[derive(Clone)]
pub struct Payer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    #[error(1)]
    InvalidAmount,
}

#[contractimpl]
impl Payer {
    pub fn pay(env: Env, from: Address, to: Address, token: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Ensure `from` is authorized
        env.require_auth(&from);

        // Create a client for the token contract and perform transfer
        let token_client = soroban_sdk::token::Client::new(env.clone(), token);
        token_client.transfer(&from, &to, amount);

        Ok(())
    }
}
