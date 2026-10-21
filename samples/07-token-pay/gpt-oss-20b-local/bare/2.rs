#![no_std]

use soroban_sdk::{contractimpl, token::Client as TokenClient, Address, Env};

#[derive(Clone)]
pub struct Payer;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
#[soroban_sdk::contracterror]
pub enum Error {
    InvalidAmount = 1,
}

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

        // Ensure the caller is authorized by `from`.
        env.require_auth(&from);

        let client = TokenClient::new(env.clone(), token);
        client.transfer(&from, &to, amount);
        Ok(())
    }
}
