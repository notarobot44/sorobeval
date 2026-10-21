#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl,
    Env, Address,
};
use soroban_sdk::token::Client as TokenClient;

#[contract]
pub struct Payer;

#[contracterror]
#[repr(u32)]
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

        // Ensure the caller authorizes the transfer.
        from.require_auth();

        // Perform the cross‑contract token transfer.
        let client = TokenClient::new(&env, &token);
        client.transfer(&from, &to, &amount);

        Ok(())
    }
}
