#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracterror]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Value,
}

#[contract]
pub struct Gated;

#[contractimpl]
impl Gated {
    /// Stores the admin in instance storage. Fails if already initialized.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        let storage = env.storage().instance();
        let existing: Option<Address> = storage.get(&DataKey::Admin);
        if existing.is_some() {
            return Err(Error::AlreadyInitialized);
        }
        storage.set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Requires authorization from the stored admin, then stores the value.
    pub fn set_value(env: Env, value: u32) -> Result<(), Error> {
        let storage = env.storage().instance();
        let admin: Address = storage.get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();
        storage.set(&DataKey::Value, &value);
        Ok(())
    }

    /// Returns the stored value, or 0 if it was never set.
    pub fn value(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Value)
            .unwrap_or(0)
    }
}
