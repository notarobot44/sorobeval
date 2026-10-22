#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, token::TokenClient, Address, Env};

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
        from.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&from, &to, &amount);

        Ok(())
    }
}
