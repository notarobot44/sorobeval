#![no_std]

use soroban_sdk::{contracterror, contractimpl, token, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contractimpl]
pub struct Treasury;

pub struct TreasuryDataKey;
impl soroban_sdk::storage::StorageKey for TreasuryDataKey {
    fn domain() -> &'static str {
        "Treasury"
    }
}

pub struct AdminKey;
impl soroban_sdk::storage::Storable for AdminKey {
    const NAME: &'static str = "Admin";
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct TokenKey;
impl soroban_sdk::storage::Storable for TokenKey {
    const NAME: &'static str = "Token";
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct CapKey;
impl soroban_sdk::storage::Storable for CapKey {
    const NAME: &'static str = "Cap";
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct SpentKey;
impl soroban_sdk::storage::Storable for SpentKey {
    const NAME: &'static str = "Spent";
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct WhitelistKey;
impl soroban_sdk::storage::Storable for WhitelistKey {
    const NAME: &'static str = "Whitelist";
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&TreasuryDataKey) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        let admin_key = AdminKey(admin);
        let token_key = TokenKey(token);
        let cap_key = CapKey(cap);
        let spent_key = SpentKey(0);

        env.storage().instance().set(&TreasuryDataKey, &admin_key);
        env.storage().instance().set(&TokenKey::NAME, &token_key);
        env.storage().instance().set(&CapKey::NAME, &cap_key);
        env.storage().instance().set(&SpentKey::NAME, &spent_key);

        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = Self::get_admin(&env)?;
        soroban_sdk::require_auth(&admin);
        let mut whitelist = env.storage().instance().get::<WhitelistKey>(&WhitelistKey).unwrap_or_default();
        if !whitelist.contains(&recipient) {
            whitelist.push(recipient);
            env.storage().instance().set(&WhitelistKey, &whitelist);
        }
        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin = Self::get_admin(&env)?;
        soroban_sdk::require_auth(&admin);

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let whitelist = env.storage().instance().get::<WhitelistKey>(&WhitelistKey).unwrap_or_default();
        if !whitelist.contains(&to) {
            return Err(Error::NotWhitelisted);
        }

        let cap = Self::cap(&env);
        let spent = Self::spent(&env);

        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        let token_address = Self::get_token(&env)?;
        let client = token::Client::new(&env, &token_address);
        client.transfer(&env.current_contract_address(), &to, &amount);

        let new_spent = spent + amount;
        env.storage().instance().set(&SpentKey::NAME, &SpentKey(new_spent));

        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        Self::get_spent(&env).0
    }

    pub fn cap(env: Env) -> i128 {
        Self::get_cap(&env).0
    }

    fn get_admin(env: &Env) -> Result<Address, Error> {
        let admin_key = env.storage().instance().get::<AdminKey>(&TreasuryDataKey).ok_or(Error::NotInitialized)?;
        Ok(admin_key.0)
    }

    fn get_token(env: &Env) -> Result<Address, Error> {
        let token_key = env.storage().instance().get::<TokenKey>(&TokenKey::NAME).ok_or(Error::NotInitialized)?;
        Ok(token_key.0)
    }

    fn get_cap(env: &Env) -> CapKey {
        env.storage().instance().get::<CapKey>(&CapKey::NAME).unwrap()
    }

    fn get_spent(env: &Env) -> SpentKey {
        env.storage().instance().get::<SpentKey>(&SpentKey::NAME).unwrap_or(SpentKey(0))
    }
}
