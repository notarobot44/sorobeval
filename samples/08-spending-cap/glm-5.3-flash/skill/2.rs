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
    Whitelist(Address),
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    /// Initializes the treasury with an admin, a token, and an immutable
    /// lifetime spending cap. Can only succeed once.
    ///
    /// NOTE: unlike the other state-changing functions, `init` intentionally
    /// performs no `require_auth` — it is typically called at deploy time by
    /// the deployer, and the `AlreadyInitialized` guard makes re-initialization
    /// impossible regardless of who calls it.
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
        Self::require_admin(&env)?;

        env.storage()
            .instance()
            .set(&DataKey::Whitelist(recipient), &true);

        Ok(())
    }

    /// Pays `amount` of the treasury's token to a whitelisted recipient,
    /// subject to the immutable lifetime cap. Admin only.
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        Self::require_admin(&env)?;

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let whitelisted: bool = env
            .storage()
            .instance()
            .get(&DataKey::Whitelist(to.clone()))
            .unwrap_or(false);
        if !whitelisted {
            return Err(Error::NotWhitelisted);
        }

        let spent: i128 = env.storage().instance().get(&DataKey::Spent).unwrap_or(0);
        let cap: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Cap)
            .ok_or(Error::NotInitialized)?;

        // Checked arithmetic guards overflow while enforcing the invariant
        // that cumulative outflow never exceeds the cap.
        let new_spent = spent.checked_add(amount).ok_or(Error::CapExceeded)?;
        if new_spent > cap {
            return Err(Error::CapExceeded);
        }

        let token_client = Client::new(&env, &Self::token_addr(&env)?);
        token_client.transfer(
            &env.current_contract_address(),
            &to,
            &amount,
        );

        env.storage().instance().set(&DataKey::Spent, &new_spent);

        Ok(())
    }

    /// Running total of funds paid out. 0 before any spend.
    pub fn spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Spent).unwrap_or(0)
    }

    /// The immutable lifetime cap set at init.
    pub fn cap(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Cap)
            .expect("treasury not initialized")
    }

    fn require_admin(env: &Env) -> Result<Address, Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        Ok(admin)
    }

    fn token_addr(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)
    }
}
