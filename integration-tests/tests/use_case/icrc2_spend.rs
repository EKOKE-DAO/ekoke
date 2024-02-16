use candid::Nat;
use integration_tests::{
    actor::{alice, alice_account, bob_account, charlie, charlie_account},
    client::IcrcLedgerClient,
    TestEnv,
};
use serial_test::serial;

#[test]
#[serial]
fn test_should_spend_approved_funds_on_ekoke() {
    let env = TestEnv::init();

    let ekoke_client = IcrcLedgerClient::new(env.ekoke_id, &env);
    let amount = Nat::from(1_000u64);

    let alice_balance = ekoke_client.icrc1_balance_of(alice_account());
    let bob_balance = ekoke_client.icrc1_balance_of(bob_account());

    // approve spend from alice to charlie
    assert!(ekoke_client
        .icrc2_approve(alice(), charlie_account(), amount.clone(), None)
        .is_ok());

    // spend approved funds
    let err = ekoke_client
        .icrc2_transfer_from(
            charlie(),
            alice_account(),
            bob_account(),
            amount.clone(),
            None,
        )
        .unwrap_err();
    println!("{:?}", err);
    assert!(ekoke_client
        .icrc2_transfer_from(
            charlie(),
            alice_account(),
            bob_account(),
            amount.clone(),
            None,
        )
        .is_ok());

    // verify balance
    assert_eq!(
        ekoke_client.icrc1_balance_of(alice_account()),
        alice_balance - Nat::from(1_0000u64)
    );
    assert_eq!(
        ekoke_client.icrc1_balance_of(bob_account()),
        bob_balance + Nat::from(1_000u64)
    );
}
