#![cfg(test)]

extern crate std;

use soroban_sdk::{contract, testutils::Address as _, Address, Env};
use stellar_event_assertion::EventAssertion;

use crate::{
    extensions::mintable::storage::mint,
    storage::{balance, total_supply},
};

#[contract]
struct MockContract;

#[test]
fn mint_works() {
    let e = Env::default();
    e.mock_all_auths();
    let address = e.register(MockContract, ());
    let account = Address::generate(&e);
    e.as_contract(&address, || {
        mint(&e, &account, 100);
        assert_eq!(balance(&e, &account), 100);
        assert_eq!(total_supply(&e), 100);

        let mut event_assert = EventAssertion::new(&e, address.clone());
        event_assert.assert_event_count(1);
        event_assert.assert_fungible_mint(&account, 100);
    });
}

/// Test that confirms the base mint implementation does NOT require
/// authorization
///
/// **IMPORTANT**: This test verifies the intentional design choice that the
/// base mint implementation doesn't include authorization controls. This is NOT
/// a security flaw but rather a design decision to give implementers
/// flexibility in how they implement authorization.
///
/// When using this function in your contracts, you MUST add your own
/// authorization controls to ensure only designated accounts can mint tokens.
#[test]
fn mint_base_implementation_has_no_auth() {
    let e = Env::default();
    // Note: we're intentionally NOT mocking any auths
    let address = e.register(MockContract, ());
    let account = Address::generate(&e);

    // This should NOT panic even without authorization
    e.as_contract(&address, || {
        mint(&e, &account, 100);
        assert_eq!(balance(&e, &account), 100);
    });
}
