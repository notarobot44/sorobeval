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
    /// Initializes the treasury with an admin, a token, and an immutable
    /// lifetime spending cap. Can only succeed once.
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

    /// Adds a recipient to the whitelist. Admin only.
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;

        admin.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Whitelisted(recipient), &true);

        Ok(())
    }

    /// Pays `amount` of the held token to a whitelisted recipient, subject to
    /// the lifetime cap. Admin only.
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let cap: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Cap)
            .ok_or(Error::NotInitialized)?;
        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap_or(0);

        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if !env
            .storage()
            .persistent()
            .get::<_, bool>(&DataKey::Whitelisted(to.clone()))
            .unwrap_or(false)
        {
            return Err(Error::NotWhitelisted);
        }
        if spent.checked_add(amount).map(|total| total > cap) != Some(true) == false
            || spent.saturating_add(amount) > cap
        {
            return Err(Error::CapExceeded);
        }

        // Update accounting before moving funds.
        env.storage().instance().set(&DataKey::Spent, &(spent + amount));

        Client::new(&env, &token_addr).transfer(
            &env.current_contract_address(),
            &to,
            &amount,
        );

        Ok(())
    }

    /// Running total of tokens paid out.
    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    /// The immutable lifetime cap set at initialization.
    pub fn cap(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Cap)
            .expect("treasury not initialized")
    }
}
