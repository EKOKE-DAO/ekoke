use candid::{Encode, Nat};
use did::ekoke::{Ekoke, EkokeResult};
use did::ekoke_reward_pool::Role;
use did::ID;
use icrc::icrc1::account::Account;
use integration_tests::actor::{admin, alice, alice_account, bob, bob_account};
use integration_tests::client::{EkokeRewardPoolClient, IcrcLedgerClient};
use integration_tests::{ekoke_to_e8s, TestEnv};

#[test]
#[serial_test::serial]
fn test_should_inspect_is_admin() {
    let env = TestEnv::init();

    assert!(env
        .update::<()>(
            env.ekoke_reward_pool_id,
            admin(),
            "admin_set_role",
            Encode!(&alice(), &Role::Admin).unwrap(),
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_fail_inspect_admin() {
    let env = TestEnv::init();
    // not an admin
    assert!(env
        .update::<()>(
            env.ekoke_reward_pool_id,
            bob(),
            "admin_set_role",
            Encode!(&alice(), &Role::Admin).unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_get_contract_reward() {
    let env = TestEnv::init();
    // is deferred canister
    assert!(env
        .update::<EkokeResult<Ekoke>>(
            env.ekoke_reward_pool_id,
            env.deferred_id,
            "get_contract_reward",
            Encode!(&Nat::from(1_u64), &10u64).unwrap(),
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_fail_inspect_get_contract_reward() {
    let env = TestEnv::init();
    // is a random guy
    assert!(env
        .update::<EkokeResult<Ekoke>>(
            env.ekoke_reward_pool_id,
            alice(),
            "get_contract_reward",
            Encode!(&Nat::from(1_u64), &10u64).unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_send_reward() {
    let env = TestEnv::init();

    let ekoke_client = EkokeRewardPoolClient::new(&env);
    let ekoke_ledger_client = IcrcLedgerClient::new(env.ekoke_ledger_id, &env);

    assert!(ekoke_ledger_client
        .icrc2_approve(
            alice(),
            Account::from(env.ekoke_reward_pool_id),
            ekoke_to_e8s(100) + 1_000u64,
            alice_account().subaccount
        )
        .is_ok());
    // reserve pool
    let contract_id = ID::from(1_u64);
    assert!(ekoke_client
        .reserve_pool(alice_account(), contract_id.clone(), ekoke_to_e8s(100))
        .is_ok());

    // inspect send reward
    assert!(env
        .update::<EkokeResult<()>>(
            env.ekoke_reward_pool_id,
            env.marketplace_id,
            "send_reward",
            Encode!(&contract_id, &ekoke_to_e8s(10), &bob_account()).unwrap(),
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_fail_inspect_send_reward() {
    let env = TestEnv::init();

    let ekoke_client = EkokeRewardPoolClient::new(&env);
    let ekoke_ledger_client = IcrcLedgerClient::new(env.ekoke_ledger_id, &env);

    assert!(ekoke_ledger_client
        .icrc2_approve(
            alice(),
            Account::from(env.ekoke_reward_pool_id),
            ekoke_to_e8s(100) + 1_000u64,
            alice_account().subaccount
        )
        .is_ok());
    // reserve pool
    let contract_id = ID::from(1_u64);
    assert!(ekoke_client
        .reserve_pool(alice_account(), contract_id.clone(), ekoke_to_e8s(100))
        .is_ok());

    // inspect send reward (fails because it's not a marketplace canister)
    assert!(env
        .update::<EkokeResult<()>>(
            env.ekoke_reward_pool_id,
            alice(),
            "send_reward",
            Encode!(&contract_id, &ekoke_to_e8s(100), &bob_account()).unwrap(),
        )
        .is_err());
}
