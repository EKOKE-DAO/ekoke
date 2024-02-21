use candid::Nat;
use did::ekoke_index::GetAccountTransactionArgs;
use integration_tests::actor::{alice, alice_account, bob_account, charlie, charlie_account};
use integration_tests::client::{EkokeArchiveClient, EkokeIndexClient, IcrcLedgerClient};
use integration_tests::TestEnv;
use serial_test::serial;

#[test]
#[serial]
fn test_should_register_transactions_into_ekoke_index() {
    const EKOKE_FEE: u64 = 10_000;
    let env = TestEnv::init();

    let ledger_client = IcrcLedgerClient::new(env.ekoke_ledger_id, &env);
    let index_client = EkokeIndexClient::new(&env);
    let archive_client = EkokeArchiveClient::new(&env);
    let amount = Nat::from(1_000u64);
    let allowance = amount.clone() + EKOKE_FEE;
    let total_spent_by_alice = allowance.clone() + EKOKE_FEE; // Fee is paid for approval as well

    let alice_balance = ledger_client.icrc1_balance_of(alice_account());
    let bob_balance = ledger_client.icrc1_balance_of(bob_account());

    // approve spend from alice to charlie
    assert!(ledger_client
        .icrc2_approve(alice(), charlie_account(), allowance.clone(), None)
        .is_ok());

    // get transactions for alice
    let transactions = index_client
        .get_account_transactions(GetAccountTransactionArgs {
            account: alice_account(),
            max_results: 100u64.into(),
            start: None,
        })
        .unwrap();
    assert_eq!(transactions.transactions.len(), 1);
    assert_eq!(&transactions.transactions[0].transaction.kind, "approve");
    assert_eq!(
        transactions.transactions[0].transaction.from().unwrap(),
        alice_account()
    );
    assert_eq!(
        transactions.transactions[0].transaction.spender().unwrap(),
        charlie_account()
    );

    let transaction = archive_client.get_transaction(0);
    assert!(transaction.is_some());

    // spend approved funds
    assert!(ledger_client
        .icrc2_transfer_from(
            charlie(),
            alice_account(),
            bob_account(),
            amount.clone(),
            None,
        )
        .is_ok());

    let transactions = index_client
        .get_account_transactions(GetAccountTransactionArgs {
            account: alice_account(),
            max_results: 100u64.into(),
            start: None,
        })
        .unwrap();
    assert_eq!(transactions.transactions.len(), 2);
    assert_eq!(&transactions.transactions[1].transaction.kind, "transfer");
    assert_eq!(
        transactions.transactions[1].transaction.from().unwrap(),
        alice_account()
    );
    assert_eq!(
        transactions.transactions[1].transaction.spender().unwrap(),
        charlie_account()
    );
    assert_eq!(
        transactions.transactions[1].transaction.to().unwrap(),
        bob_account()
    );

    let transaction = archive_client.get_transaction(1);
    assert!(transaction.is_some());

    // verify balance
    assert_eq!(
        ledger_client.icrc1_balance_of(alice_account()),
        alice_balance - total_spent_by_alice
    );
    assert_eq!(
        ledger_client.icrc1_balance_of(bob_account()),
        bob_balance + amount
    );

    let transaction = archive_client.get_transaction(2);
    assert!(transaction.is_none());
}
