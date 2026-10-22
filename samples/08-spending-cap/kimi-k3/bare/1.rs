#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, token, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contract]
pub struct Treasury;

const ADMIN: &str = "admin";
const TOKEN: &str = "token";
const CAP: &str = "cap";
const SPENT: &str = "spent";
const WL: &str = "wl";

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOKEN, &token);
        env.storage().instance().set(&CAP, &cap);
        env.storage().instance().set(&SPENT, &0i128);
        Ok(())
    }

    fn admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&ADMIN)
            .ok_or(Error::NotInitialized)
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        Self::admin(&env)?.require_auth();
        env.storage().persistent().set(&(WL, recipient), &true);
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        Self::admin(&env)?.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !env
            .storage()
            .persistent()
            .get(&(WL, to.clone()))
            .unwrap_or(false)
        {
            return Err(Error::NotWhitelisted);
        }
        let cap: i128 = env.storage().instance().get(&CAP).ok_or(Error::NotInitialized)?;
        let spent: i128 = env.storage().instance().get(&SPENT).unwrap_or(0);
        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_spent > cap {
            return Err(Error::CapExceeded);
        }
        let token_addr: Address = env
            .storage()
            .instance()
            .get(&TOKEN)
            .ok_or(Error::NotInitialized)?;
        token::Client::new(&env, &token_addr).transfer(
            &env.current_contract_address(),
            &to,
            &amount,
        );
        env.storage().instance().set(&SPENT, &new_spent);
        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&SPENT).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage().instance().get(&CAP).unwrap_or(0)
    }
}
