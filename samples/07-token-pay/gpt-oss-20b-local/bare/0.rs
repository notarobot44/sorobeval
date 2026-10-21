#![no_std]

use soroban_sdk::{contracterror, contractimpl, Address, Env, IntoVal};

#[derive(Clone)]
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

        // Ensure the caller is authorized as `from`
        env.invocation().require_auth(&from);

        // Call the token contract's transfer function
        let client = soroban_sdk::token::Client::new(env.clone(), token.clone());
        client.transfer(&env, &from, &to, amount);

        Ok(())
    }
}
