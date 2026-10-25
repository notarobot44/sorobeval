#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, Address, BytesN, Env, Symbol,
};

const ADMIN_KEY: &[u8; 4] = b"ADMN";
const TOKEN_KEY: &[u8; 5] = b"TOKEN";
const CAP_KEY: &[u8; 3] = b"CAP";
const SPENT_KEY: &[u8; 5] = b"SPNT";

#[contracterror]
pub enum Error {
    #[msg("Contract already initialized")]
    AlreadyInitialized = 1,
    #[msg("Invalid amount")]
    InvalidAmount = 2,
    #[msg("Recipient not whitelisted")]
    NotWhitelisted = 3,
    #[msg("Cap exceeded")]
    CapExceeded = 4,
    #[msg("Contract not initialized")]
    NotInitialized = 5,
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    /// Initialize the treasury contract.
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        // Ensure not already initialized
        if env.storage().has(ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        env.storage()
            .set(ADMIN_KEY, &admin.clone());
        env.storage()
            .set(TOKEN_KEY, &token.clone());
        env.storage().set(CAP_KEY, &cap);
        env.storage().set(SPENT_KEY, &0i128);

        Ok(())
    }

    /// Add a recipient to the whitelist. Admin only.
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let admin = env
            .storage()
            .get::<_, Address>(ADMIN_KEY)
            .ok_or(Error::NotInitialized)?;
        env.require_auth(&admin);

        // Use a composite key for the whitelist entry
        let key = Self::whitelist_key(&recipient);
        env.storage().set(&key, &true);
        Ok(())
    }

    /// Spend tokens to a whitelisted address. Admin only.
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let admin = env
            .storage()
            .get::<_, Address>(ADMIN_KEY)
            .ok_or(Error::NotInitialized)?;
        env.require_auth(&admin);

        // Verify whitelisted
        let key = Self::whitelist_key(&to);
        if !env.storage().has(&key) {
            return Err(Error::NotWhitelisted);
        }

        // Check cap
        let cap: i128 = env.storage().get(CAP_KEY).ok_or(Error::NotInitialized)?;
        let spent: i128 = env.storage().get(SPENT_KEY).unwrap_or(0i128);
        if spent + amount > cap {
            return Err(Error::CapExceeded);
        }

        // Transfer tokens
        let token_addr: Address = env.storage().get(TOKEN_KEY).ok_or(Error::NotInitialized)?;
        soroban_sdk::token::Client::new(&env, &token_addr)
            .transfer(
                &env.current_contract_address(),
                &to,
                amount,
            )
            .map_err(|_| Error::CapExceeded)?; // Treat transfer failure as cap exceeded

        // Update spent
        env.storage().set(SPENT_KEY, &(spent + amount));
        Ok(())
    }

    /// Return the total amount spent so far.
    pub fn spent(env: Env) -> i128 {
        env.storage()
            .get::<_, i128>(SPENT_KEY)
            .unwrap_or(0i128)
    }

    /// Return the spending cap set at initialization.
    pub fn cap(env: Env) -> i128 {
        env.storage().get::<_, i128>(CAP_KEY).unwrap_or(0i128)
    }

    // Helper to create a unique key for each whitelisted address
    fn whitelist_key(recipient: &Address) -> BytesN<32> {
        let mut data = [0u8; 32];
        recipient.as_ref().copy_to_slice(&mut data);
        BytesN::from_array(&data)
    }
}
