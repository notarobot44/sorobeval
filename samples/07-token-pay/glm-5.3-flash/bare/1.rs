#![no_std]

use soroban_sdk::{contract, contracterror, token::Client, Address, Env};

#[contracterror]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
}

#[contract]
pub struct Payer;

#[contractimpl]
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
        from.require_auth();
        let client = Client::new(&env, &token);
        client.transfer(&from, &to, &amount);
        Ok(())
    }
}
