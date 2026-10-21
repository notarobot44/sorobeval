#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, i128, Symbol};

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
pub struct TreasuryState {
    pub admin: Address,
    pub token: Address,
    pub cap: i128,
    pub spent: i128,
    pub whitelist: Vec<Address>,
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        let state_key = Symbol::short("state");
        
        if env.data().has(&state_key) {
            return Err(Error::AlreadyInitialized);
        }
        
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        let mut state = TreasuryState {
            admin,
            token,
            cap,
            spent: 0,
            whitelist: Vec::new(&env),
        };
        
        env.data().set(&state_key, state);
        Ok(())
    }
    
    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let state_key = Symbol::short("state");
        let mut state = env.data().get::<_, TreasuryState>(state_key).ok_or(Error::NotInitialized)?;
        
        // Require admin authorization
        state.admin.require_auth();
        
        state.whitelist.push_back(&recipient);
        env.data().set(&state_key, state);
        Ok(())
    }
    
    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let state_key = Symbol::short("state");
        let mut state = env.data().get::<_, TreasuryState>(state_key).ok_or(Error::NotInitialized)?;
        
        // Require admin authorization
        state.admin.require_auth();
        
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        // Check if recipient is whitelisted
        let mut found = false;
        for i in 0..state.whitelist.len() {
            if state.whitelist.get(i).unwrap() == to {
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(Error::NotWhitelisted);
        }
        
        // Check if spending would exceed cap
        if state.spent + amount > state.cap {
            return Err(Error::CapExceeded);
        }
        
        // Transfer token from contract to recipient
        let token_client = soroban_sdk::token::Client::new(&env, &state.token);
        token_client.transfer(&env.current_contract_address(), &to, &amount);
        
        // Update spent amount
        state.spent += amount;
        env.data().set(&state_key, state);
        Ok(())
    }
    
    pub fn spent(env: Env) -> i128 {
        let state_key = Symbol::short("state");
        let state = env.data().get::<_, TreasuryState>(state_key).unwrap();
        state.spent
    }
    
    pub fn cap(env: Env) -> i128 {
        let state_key = Symbol::short("state");
        let state = env.data().get::<_, TreasuryState>(state_key).unwrap();
        state.cap
    }
}
