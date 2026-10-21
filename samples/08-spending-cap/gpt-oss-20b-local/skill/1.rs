#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    token::Client as TokenClient, Address, Env,
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelist(Address),
}

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

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    /// Initialize the treasury. Can only be called once.
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        // Ensure not already initialized
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        // Cap must be positive
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        // spent starts at zero
        env.storage().instance().set(&DataKey::Spent, &0i128);
        Ok(())
    }

    /// Add a recipient to the whitelist. Admin only.
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = env
            .storage()
            .instance()
            .get::<Address>(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        // Store a marker for the whitelisted address
        env.storage()
            .persistent()
            .set(&DataKey::Whitelist(recipient), &true);
        Ok(())
    }

    /// Spend tokens to a whitelisted recipient. Admin only.
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin = env
            .storage()
            .instance()
            .get::<Address>(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Check whitelist
        if !env
            .storage()
            .persistent()
            .has(&DataKey::Whitelist(to))
        {
            return Err(Error::NotWhitelisted);
        }

        let cap = env
            .storage()
            .instance()
            .get::<i128>(&DataKey::Cap)
            .ok_or(Error::NotInitialized)?;
        let spent = env
            .storage()
            .instance()
            .get::<i128>(&DataKey::Spent)
            .unwrap_or(0);

        // Ensure cap not exceeded
        if spent.checked_add(amount).map_or(true, |new_total| new_total > cap) {
            return Err(Error::CapExceeded);
        }

        // Transfer the token from this contract to the recipient
        let token_addr = env
            .storage()
            .instance()
            .get::<Address>(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let client = TokenClient::new(&env, &token_addr);
        client.transfer(
            &env.current_contract_address(),
            &to,
            &amount,
        );

        // Update spent total
        env.storage()
            .instance()
            .set(&DataKey::Spent, &(spent + amount));
        Ok(())
    }

    /// Return the total spent so far.
    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get::<i128>(&DataKey::Spent).unwrap_or(0)
    }

    /// Return the cap set at initialization.
    pub fn cap(env: Env) -> i128 {
        env.storage()
            .instance()
            .get::<i128>(&DataKey::Cap)
            .unwrap_or(0)
    }
}
