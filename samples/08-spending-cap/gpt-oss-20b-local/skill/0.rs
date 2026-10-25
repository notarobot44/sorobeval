#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Address, Env, symbol_short,
    token::Client as TokenClient,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
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
    // ---- Initialization ----------------------------------------------------

    /// Initializes the treasury. Must be called once.
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Cap, &cap);
        env.storage().instance().set(&DataKey::Spent, &0i128);

        Ok(())
    }

    // ---- Whitelisting -----------------------------------------------------

    /// Adds a recipient to the whitelist. Only callable by admin.
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = get_admin_or_err(&env)?;
        admin.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Whitelist(recipient), &());
        Ok(())
    }

    // ---- Spending ----------------------------------------------------------

    /// Transfers tokens to a whitelisted address, respecting the cap.
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin = get_admin_or_err(&env)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Check whitelist
        if !env.storage().instance().has(&DataKey::Whitelist(to)) {
            return Err(Error::NotWhitelisted);
        }

        // Enforce cap
        let cap: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Cap)
            .ok_or(Error::NotInitialized)?;
        let spent: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Spent)
            .unwrap_or(0);
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        // Transfer tokens from contract to recipient
        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let client = TokenClient::new(&env, &token_addr);
        client.transfer(
            &env.current_contract_address(),
            &to,
            &amount,
        );

        // Update spent
        env.storage()
            .instance()
            .set(&DataKey::Spent, &(spent + amount));

        Ok(())
    }

    // ---- View helpers -----------------------------------------------------

    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Cap)
            .unwrap_or(0)
    }
}

// ---------------------------------------------------------------------------

fn get_admin_or_err(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)
}
