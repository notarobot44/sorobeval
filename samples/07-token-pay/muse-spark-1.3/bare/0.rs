#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, token::Client as TokenClient, Address, Env};

#[contract]
pub struct Payer;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
}

#[contractimpl]
impl Payer {
    pub fn pay(env: Env, from: Address, to: Address, token: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        from.require_auth();
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&from, &to, &amount);
        Ok(())
    }
}
