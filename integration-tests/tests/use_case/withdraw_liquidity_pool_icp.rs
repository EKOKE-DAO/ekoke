use candid::Nat;
use integration_tests::actor::bob_account;
use integration_tests::client::{EkokeLiquidityPoolClient, IcrcLedgerClient};
use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_as_seller_i_should_withdraw_contract_deposit_after_being_paid() {
    let env = TestEnv::init();
    let icp_ledger_client = IcrcLedgerClient::new(env.icp_ledger_id, &env);
    let ekoke_liquidity_pool_client = EkokeLiquidityPoolClient::from(&env);

    // withdraw icp to bob
    let current_bob_balance = icp_ledger_client.icrc1_balance_of(bob_account());
    let current_pool_balance =
        icp_ledger_client.icrc1_balance_of(env.ekoke_liquidity_pool_id.into());

    let icp_fee = icp_ledger_client.icrc1_fee();

    let withdraw_amount = Nat::from(1_000_000_000u64);

    let expected_bob_balance = current_bob_balance + withdraw_amount.clone();
    let expected_pool_balance = current_pool_balance - withdraw_amount.clone() - icp_fee;

    assert!(ekoke_liquidity_pool_client
        .admin_withdraw_icp(bob_account(), withdraw_amount)
        .is_ok());

    let new_bob_balance = icp_ledger_client.icrc1_balance_of(bob_account());
    let new_pool_balance = icp_ledger_client.icrc1_balance_of(env.ekoke_liquidity_pool_id.into());

    assert_eq!(new_bob_balance, expected_bob_balance);
    assert_eq!(new_pool_balance, expected_pool_balance);
}
