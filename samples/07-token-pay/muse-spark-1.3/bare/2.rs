#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, token::Client, Address, Env};

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
        Client::new(&env, &token).transfer(&from, &to, &amount);
        Ok(())
    }
}
