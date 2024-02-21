use candid::Encode;
use did::ekoke_index::{Transaction, Transfer};
use integration_tests::actor::{alice_account, bob, bob_account};
use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_should_inspect_is_ekoke_ledger() {
    let env = TestEnv::init();

    assert!(env
        .update::<()>(
            env.ekoke_index_id,
            env.ekoke_ledger_id,
            "commit",
            Encode!(&transaction()).unwrap(),
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_fail_inspect_is_ekoke_ledger() {
    let env = TestEnv::init();
    // not an admin
    assert!(env
        .update::<()>(
            env.ekoke_index_id,
            bob(),
            "commit",
            Encode!(&transaction()).unwrap(),
        )
        .is_err());
}

fn transaction() -> Transaction {
    Transaction {
        kind: "transfer".to_string(),
        mint: None,
        burn: None,
        transfer: Some(Transfer {
            amount: 100_u64.into(),
            from: alice_account(),
            to: bob_account(),
            spender: None,
            memo: None,
            created_at_time: None,
            fee: None,
        }),
        approve: None,
        timestamp: 0,
    }
}
