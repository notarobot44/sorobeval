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
    /// Transfers `amount` of `token` from `from` to `to`.
    ///
    /// Requires `from`'s authorization. Returns `Error::InvalidAmount`
    /// if `amount <= 0` without touching the token contract.
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

        let client = TokenClient::new(&env, &token);
        client.transfer(&from, &to, &amount);

        Ok(())
    }
}
