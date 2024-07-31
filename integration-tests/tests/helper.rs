use icrc::icrc1::account::Account;
use integration_tests::client::IcrcLedgerClient;
use integration_tests::TestEnv;

pub fn contract_deposit(test_env: &TestEnv, deposit_account: Account, amount: u64) {
    let icp_ledger = IcrcLedgerClient::new(test_env.icp_ledger_id, test_env);
    let fee = icp_ledger.icrc1_fee();
    let amount = amount + fee;

    icp_ledger
        .icrc2_approve(
            deposit_account.owner,
            test_env.deferred_id.into(),
            amount,
            None,
        )
        .expect("contract deposit approve failed");
}
