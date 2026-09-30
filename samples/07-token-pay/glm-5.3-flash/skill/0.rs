#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl,
    token::Client as TokenClient,
    Address, Env, MuxedAddress,
};

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

        let token_client = TokenClient::new(&env, &token);
        let to_muxed: MuxedAddress = to.into();
        token_client.transfer(&from, &to_muxed, &amount);

        Ok(())
    }
}
