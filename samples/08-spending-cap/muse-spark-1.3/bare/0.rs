#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token::Client, Address, Env};

#[contract]
pub struct Treasury;

#[contracttype]
enum DataKey {
    Admin,
    Token,
    Cap,
    Spent,
    Whitelisted(Address),
}

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
impl Treasury {
    pub fn init(env: Env, admin: Address, token: Address, cap: i128) -> Result<(), Error> {
        admin.require_auth();
        if env.storage().instance().has(&DataKey::Admin) {
           