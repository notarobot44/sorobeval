#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env, i128};

#[contracterror]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
}

pub struct Payer;

#[contract]
impl Payer {
    #[contractimpl]
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
        
        let token_client = soroban_sdk::token::Client::new(&env, &token);
        token_client.transfer(&from, &to, &amount);
        
        Ok(())
    }
}
