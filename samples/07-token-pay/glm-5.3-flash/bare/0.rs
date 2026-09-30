#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, token::Client as TokenClient, Address, Env,
};

/// Contract error codes.
#[contracterror]
#[repr(u32)]
pub enum Error {
    /// The requested payment amount was zero or negative.
    InvalidAmount = 1,
}

/// A simple cross-contract payment helper that moves tokens between accounts.
#[contract]
pub struct Payer;

#[contractimpl]
impl Payer {
    /// Transfers `amount` units of `token` from `from` to `to`.
    ///
    /// Authorization must be provided by `from`. If `amount <= 0`, returns
    /// [`Error::InvalidAmount`] without invoking the token contract.
    pub fn pay(
        env: Env,
        from: Address,
        to: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), Error> {
        // Reject invalid amounts up front, before touching the token contract.
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Ensure the source account authorizes this specific spend.
        from.require_auth();

        // Perform the transfer through the token contract's client wrapper.
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&from, &to, &amount);

        Ok(())
    }
}
