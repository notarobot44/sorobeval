#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token::Client, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelisted(Address),
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    /// Initializes the treasury with an admin, a token and an immutable
    /// lifetime spending cap. Can only succeed once.
    ///
    /// Note: the cap is stored once and never rewritten by any function,
    /// making it effectively immutable after initialization.
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        env.storage().instance().set(&DataKey::Spent, &0i128);

        Ok(())
    }

    /// Adds a recipient to the whitelist. Requires the admin's signature.
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = Self::_admin(&env)?;
        admin.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Whitelisted(recipient), &());

        Ok(())
    }

    /// Pays `amount` of the held token to a whitelisted recipient, subject
    /// to the lifetime cap. Requires the admin's signature.
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = Self::_admin(&env)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !Self::_is_whitelisted(&env, &to)? {
            return Err(Error::NotWhitelisted);
        }

        let cap: i128 = Self::_cap(&env)?;
        let spent: i128 = Self::_spent(&env);
        if checked_add(spent, amount).map_or(true, |total| total > cap) {
            return Err(Error::CapExceeded);
        }

        let token: Address = Self::_token(&env)?;
        let client = Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &to, &amount);

        // Update the running total only after the transfer succeeded.
        env.storage()
            .instance()
            .set(&DataKey::Spent, &(spent + amount));

        Ok(())
    }

    /// Running total of funds paid out. Returns 0 before any spend.
    pub fn spent(env: Env) -> i128 {
        Self::_spent(&env)
    }

    /// The lifetime cap fixed at initialization.
    pub fn cap(env: Env) -> i128 {
        Self::_cap(&env).unwrap_or(0)
    }

    // ---- internal helpers -------------------------------------------------

    fn _admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get::<_, Address>(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    fn _token(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get::<_, Address>(&DataKey::Token)
            .ok_or(Error::NotInitialized)
    }

    fn _cap(env: &Env) -> Result<i128, Error> {
        env.storage()
            .instance()
            .get::<_, i128>(&DataKey::Cap)
            .ok_or(Error::NotInitialized)
    }

    fn _spent(env: &Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    fn _is_whitelisted(env: &Env, who: &Address) -> Result<bool, Error> {
        Self::_admin(env)?;
        Ok(env
            .storage()
            .persistent()
            .has(&DataKey::Whitelisted(who.clone())))
    }
}

/// Checked addition used to guard the cap computation against overflow.
fn checked_add(a: i128, b: i128) -> Option<i128> {
    a.checked_add(b)
}
