#![no_std]

use soroban_sdk::{
    auth::Auth,
    contract::{contractimpl, Contract},
    token,
    Env, IntoVal, Symbol,
};

#[derive(Clone)]
pub struct Payer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Error {
    #[error(1)]
    InvalidAmount,
}

#[contractimpl]
impl Payer {
    pub fn pay(
        env: Env,
        from: soroban_sdk::Address,
        to: soroban_sdk::Address,
        token: soroban_sdk::Address,
        amount: i128,
    ) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Require authorization from the `from` address
        env.authorize(&Auth::new(from.clone(), Symbol::new("PAY")));

        // Perform the token transfer
        let client = token::Client::new(&env, &token);
        client.transfer(&from, &to, amount);

        Ok(())
    }
}
