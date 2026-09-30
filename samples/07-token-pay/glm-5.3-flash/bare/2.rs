#![no_std]

use soroban_sdk::{contractclient, contracterror, contracttype, Address, Env};

#[contracterror]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
}

#[contracttype]
pub struct Payer;

impl Payer {
    pub fn pay(
        env: &Env,
        from: Address,
        to: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), Error> {
        from.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let client = TokenClient::new(env, &token);
        client.transfer(&from, &to, &amount);

        Ok(())
    }
}
