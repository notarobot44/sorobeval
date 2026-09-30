#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

struct Fx<'a> {
    env: Env,
    payer: PayerClient<'a>,
    token: token::Client<'a>,
    token_id: Address,
    alice: Address,
    bob: Address,
}

fn setup() -> Fx<'static> {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let token_id = sac.address();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    token::StellarAssetClient::new(&env, &token_id).mint(&alice, &1_000);
    let payer_id = env.register(Payer, ());
    Fx {
        payer: PayerClient::new(&env, &payer_id),
        token: token::Client::new(&env, &token_id),
        env,
        token_id,
        alice,
        bob,
    }
}

#[test]
fn transfers_between_accounts() {
    let f = setup();
    f.payer.pay(&f.alice, &f.bob, &f.token_id, &250);
    assert_eq!(f.token.balance(&f.alice), 750);
    assert_eq!(f.token.balance(&f.bob), 250);
}

#[test]
fn rejects_non_positive_amount() {
    let f = setup();
    assert_eq!(f.payer.try_pay(&f.alice, &f.bob, &f.token_id, &0), Err(Ok(Error::InvalidAmount)));
    assert_eq!(f.payer.try_pay(&f.alice, &f.bob, &f.token_id, &-5), Err(Ok(Error::InvalidAmount)));
    assert_eq!(f.token.balance(&f.alice), 1_000);
}

#[test]
fn insufficient_balance_fails_and_moves_nothing() {
    let f = setup();
    assert!(f.payer.try_pay(&f.alice, &f.bob, &f.token_id, &5_000).is_err());
    assert_eq!(f.token.balance(&f.alice), 1_000);
    assert_eq!(f.token.balance(&f.bob), 0);
    let _ = &f.env;
}
