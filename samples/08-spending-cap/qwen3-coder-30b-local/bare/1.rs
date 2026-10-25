#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, BytesN, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotWhitelisted = 3,
    CapExceeded = 4,
    NotInitialized = 5,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Treasury {
    pub admin: Address,
    pub token: Address,
    pub cap: i128,
    pub spent: i128,
    pub whitelist: Vec<Address>,
}

#[contract]
pub struct TreasuryContract;

#[contractimpl]
impl TreasuryContract {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        let treasury_key = BytesN::from_array(&env, b"treasury");
        if env.data().has(&treasury_key) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        let mut treasury = Treasury {
            admin: admin.clone(),
            token: token.clone(),
            cap,
            spent: 0,
            whitelist: Vec::new(&env),
        };
        
        env.data().set(&treasury_key, treasury);
        Ok(())
    }
    
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let treasury_key = BytesN::from_array(&env, b"treasury");
        let mut treasury: Treasury = env.data().get(&treasury_key).unwrap();
        treasury.admin.require_auth();
        
        treasury.whitelist.push_back(recipient);
        env.data().set(&treasury_key, treasury);
        Ok(())
    }
    
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let treasury_key = BytesN::from_array(&env, b"treasury");
        let mut treasury: Treasury = env.data().get(&treasury_key).unwrap();
        treasury.admin.require_auth();
        
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        if !treasury.whitelist.contains(&to) {
            return Err(Error::NotWhitelisted);
        }
        
        if treasury.spent + amount > treasury.cap {
            return Err(Error::CapExceeded);
        }
        
        let token_client = soroban_sdk::token::Client::new(&env, &treasury.token);
        token_client.transfer(&env.current_contract_address(), &to, &amount);
        
        treasury.spent += amount;
        env.data().set(&treasury_key, treasury);
        Ok(())
    }
    
    pub fn spent(env: Env) -> Result<i128, Error> {
        let treasury_key = BytesN::from_array(&env, b"treasury");
        let treasury: Treasury = env.data().get(&treasury_key).unwrap();
        Ok(treasury.spent)
    }
    
    pub fn cap(env: Env) -> Result<i128, Error> {
        let treasury_key = BytesN::from_array(&env, b"treasury");
        let treasury: Treasury = env.data().get(&treasury_key).unwrap();
        Ok(treasury.cap)
    }
}
