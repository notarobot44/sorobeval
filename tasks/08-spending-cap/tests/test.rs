#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

struct Fx<'a> {
    env: Env,
    t: TreasuryClient<'a>,
    tok: token::Client<'a>,
    admin: Address,
    payee: Address,
    stranger: Address,
}

fn setup(cap: i128) -> Fx<'static> {
    let env = Env::default();
    env.mock_all_auths();
    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let tok_id = sac.address();
    let tid = env.register(Treasury, ());
    let t = TreasuryClient::new(&env, &tid);
    let admin = Address::generate(&env);
    let payee = Address::generate(&env);
    let stranger = Address::generate(&env);
    token::StellarAssetClient::new(&env, &tok_id).mint(&tid, &10_000);
    t.init(&admin, &tok_id, &cap);
    t.whitelist(&payee);
    Fx { t, tok: token::Client::new(&env, &tok_id), env, admin, payee, stranger }
}

#[test]
fn spends_within_cap() {
    let f = setup(1_000);
    f.t.spend(&f.payee, &400);
    f.t.spend(&f.payee, &600);
    assert_eq!(f.t.spent(), 1_000);
    assert_eq!(f.tok.balance(&f.payee), 1_000);
    assert_eq!(f.t.cap(), 1_000);
}

#[test]
fn cumulative_outflow_never_exceeds_cap() {
    let f = setup(1_000);
    f.t.spend(&f.payee, &999);
    assert_eq!(f.t.try_spend(&f.payee, &2), Err(Ok(Error::CapExceeded)));
    assert_eq!(f.t.spent(), 999);
    assert_eq!(f.tok.balance(&f.payee), 999);
    // one more unit is still allowed, exactly at the cap
    f.t.spend(&f.payee, &1);
    assert_eq!(f.t.try_spend(&f.payee, &1), Err(Ok(Error::CapExceeded)));
}

#[test]
fn non_whitelisted_recipient_rejected() {
    let f = setup(1_000);
    assert_eq!(f.t.try_spend(&f.stranger, &1), Err(Ok(Error::NotWhitelisted)));
    assert_eq!(f.tok.balance(&f.stranger), 0);
    assert_eq!(f.t.spent(), 0);
}

#[test]
fn non_positive_amount_rejected() {
    let f = setup(1_000);
    assert_eq!(f.t.try_spend(&f.payee, &0), Err(Ok(Error::InvalidAmount)));
    assert_eq!(f.t.try_spend(&f.payee, &-1), Err(Ok(Error::InvalidAmount)));
}

#[test]
fn init_guards() {
    let f = setup(1_000);
    assert_eq!(f.t.try_init(&f.admin, &f.tok.address, &5), Err(Ok(Error::AlreadyInitialized)));
    let env = Env::default();
    env.mock_all_auths();
    let t2 = TreasuryClient::new(&env, &env.register(Treasury, ()));
    let a = Address::generate(&env);
    assert_eq!(t2.try_init(&a, &a, &0), Err(Ok(Error::InvalidAmount)));
    assert_eq!(t2.try_spend(&a, &1), Err(Ok(Error::NotInitialized)));
}

#[test]
fn unauthorized_calls_rejected() {
    let f = setup(1_000);
    f.env.set_auths(&[]);
    assert!(f.t.try_spend(&f.payee, &1).is_err());
    assert!(f.t.try_whitelist(&f.stranger).is_err());
    assert_eq!(f.t.spent(), 0);
    assert_eq!(f.tok.balance(&f.payee), 0);
}
