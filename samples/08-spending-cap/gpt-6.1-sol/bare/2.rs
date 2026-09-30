#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address, Env,
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
#[derive(Clone)]
enum DataKey {
    State,
    Whitelisted(Address),
}

#[contracttype]
#[derive(Clone)]
struct State {
    admin: Address,
    token: Address,
    cap: i128,
    spent: i128,
}

fn load_state(env: &Env) -> Result<State, Error> {
    env.storage()
        .instance()
        .get(&DataKey::State)
        .ok_or(Error::NotInitialized)
}

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::State) {
            return Err(Error::AlreadyInitialized);
        }
        if cap <= 0 {
            return Err(Error::InvalidAmount);
        }

        admin.require_auth();

        env.storage().instance().set(
            &DataKey::State,
            &State {
                admin,
                token,
                cap,
                spent: 0,
            },
        );

        Ok(())
    }

    pub fn whitelist(env: Env, recipient: Address) -> Result<(), Error> {
        let state = load_state(&env)?;
        state.admin.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Whitelisted(recipient), &true);

        Ok(())
    }

    pub fn spend(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let mut state = load_state(&env)?;
        state.admin.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        if !env
            .storage()
            .instance()
            .get::<DataKey, bool>(&DataKey::Whitelisted(to.clone()))
            .unwrap_or(false)
        {
            return Err(Error::NotWhitelisted);
        }

        let new_spent = state
            .spent
            .checked_add(amount)
            .ok_or(Error::CapExceeded)?;

        if new_spent > state.cap {
            return Err(Error::CapExceeded);
        }

        // Reserve the spending allowance before the external call. A failed
        // transfer rolls back this update along with the entire invocation.
        state.spent = new_spent;
        env.storage().instance().set(&DataKey::State, &state);

        token::Client::new(&env, &state.token).transfer(
            &env.current_contract_address(),
            &to,
            &amount,
        );

        Ok(())
    }

    pub fn spent(env: Env) -> i128 {
        env.storage()
            .instance()
            .get::<DataKey, State>(&DataKey::State)
            .map(|state| state.spent)
            .unwrap_or(0)
    }

    pub fn cap(env: Env) -> i128 {
        match load_state(&env) {
            Ok(state) => state.cap,
            Err(error) => panic_with_error!(&env, error),
        }
    }
}
