use candid::{Encode, Nat};
use did::ekoke::{EkokeResult, PicoEkoke};
use did::ID;
use icrc::icrc1::transfer::{TransferArg, TransferError};
use icrc::icrc2::approve::{ApproveArgs, ApproveError};
use icrc::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use integration_tests::actor::{admin, alice, alice_account, bob, bob_account};
use integration_tests::client::EkokeClient;
use integration_tests::{ekoke_to_picoekoke, TestEnv};

#[test]
#[serial_test::serial]
fn test_should_inspect_is_admin() {
    let env = TestEnv::init();

    assert!(env
        .update::<()>(
            env.ekoke_id,
            admin(),
            "admin_set_xrc_canister",
            Encode!(&env.xrc_id).unwrap(),
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
            env.ekoke_id,
            bob(),
            "admin_set_xrc_canister",
            Encode!(&env.xrc_id).unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_get_contract_reward() {
    let env = TestEnv::init();
    // is deferred canister
    assert!(env
        .update::<EkokeResult<PicoEkoke>>(
            env.ekoke_id,
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
        .update::<EkokeResult<PicoEkoke>>(
            env.ekoke_id,
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

    let ekoke_client = EkokeClient::new(&env);
    // reserve pool
    let contract_id = ID::from(1_u64);
    assert!(ekoke_client
        .reserve_pool(
            alice_account(),
            contract_id.clone(),
            ekoke_to_picoekoke(100)
        )
        .is_ok());

    // inspect send reward
    assert!(env
        .update::<EkokeResult<()>>(
            env.ekoke_id,
            env.marketplace_id,
            "send_reward",
            Encode!(&contract_id, &ekoke_to_picoekoke(10), &bob_account()).unwrap(),
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_fail_inspect_send_reward() {
    let env = TestEnv::init();

    let ekoke_client = EkokeClient::new(&env);
    // reserve pool
    let contract_id = ID::from(1_u64);
    assert!(ekoke_client
        .reserve_pool(
            alice_account(),
            contract_id.clone(),
            ekoke_to_picoekoke(100)
        )
        .is_ok());

    // inspect send reward (fails because it's not a marketplace canister)
    assert!(env
        .update::<EkokeResult<()>>(
            env.ekoke_id,
            alice(),
            "send_reward",
            Encode!(&contract_id, &ekoke_to_picoekoke(100), &bob_account()).unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_icrc1_transfer() {
    let env = TestEnv::init();

    let transfer = TransferArg {
        from_subaccount: None,
        to: bob_account(),
        fee: None,
        created_at_time: None,
        memo: None,
        amount: 100u64.into(),
    };

    // inspect icrc1_transfer
    assert!(env
        .update::<Result<Nat, TransferError>>(
            env.ekoke_id,
            alice(),
            "icrc1_transfer",
            Encode!(&transfer).unwrap()
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_fail_inspect_icrc1_transfer() {
    let env = TestEnv::init();

    // bad fee
    let transfer = TransferArg {
        from_subaccount: None,
        to: bob_account(),
        fee: Some(1u64.into()),
        created_at_time: None,
        memo: None,
        amount: 100u64.into(),
    };

    // inspect icrc1_transfer
    assert!(env
        .update::<Result<Nat, TransferError>>(
            env.ekoke_id,
            alice(),
            "icrc1_transfer",
            Encode!(&transfer).unwrap()
        )
        .is_err());

    // Bad memo
    let transfer = TransferArg {
        from_subaccount: None,
        to: bob_account(),
        fee: None,
        created_at_time: None,
        memo: Some(vec![0x00, 0x01].into()),
        amount: 100u64.into(),
    };

    // inspect icrc1_transfer
    assert!(env
        .update::<Result<Nat, TransferError>>(
            env.ekoke_id,
            alice(),
            "icrc1_transfer",
            Encode!(&transfer).unwrap()
        )
        .is_err());

    // too old
    let transfer = TransferArg {
        from_subaccount: None,
        to: bob_account(),
        fee: None,
        created_at_time: Some(0),
        memo: None,
        amount: 100u64.into(),
    };

    // inspect icrc1_transfer
    assert!(env
        .update::<Result<Nat, TransferError>>(
            env.ekoke_id,
            alice(),
            "icrc1_transfer",
            Encode!(&transfer).unwrap()
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_icrc2_approve() {
    let env = TestEnv::init();

    let args = ApproveArgs {
        spender: bob_account(),
        from_subaccount: None,
        amount: 100u64.into(),
        expected_allowance: None,
        expires_at: None,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    // inspect icrc2_approve
    assert!(env
        .update::<Result<Nat, ApproveError>>(
            env.ekoke_id,
            alice(),
            "icrc2_approve",
            Encode!(&args).unwrap()
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_fail_inspect_icrc2_approve() {
    let env = TestEnv::init();

    // inspect spender is caller
    let args = ApproveArgs {
        spender: bob_account(),
        from_subaccount: None,
        amount: 100u64.into(),
        expected_allowance: None,
        expires_at: None,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    assert!(env
        .update::<Result<Nat, ApproveError>>(
            env.ekoke_id,
            bob(),
            "icrc2_approve",
            Encode!(&args).unwrap()
        )
        .is_err());

    // inspect fee too low
    let args = ApproveArgs {
        spender: bob_account(),
        from_subaccount: None,
        amount: 100u64.into(),
        expected_allowance: None,
        expires_at: None,
        fee: Some(1u64.into()),
        memo: None,
        created_at_time: None,
    };

    assert!(env
        .update::<Result<Nat, ApproveError>>(
            env.ekoke_id,
            alice(),
            "icrc2_approve",
            Encode!(&args).unwrap()
        )
        .is_err());

    // bad memo
    let args = ApproveArgs {
        spender: bob_account(),
        from_subaccount: None,
        amount: 100u64.into(),
        expected_allowance: None,
        expires_at: None,
        fee: None,
        memo: Some(vec![0x00, 0x01].into()),
        created_at_time: None,
    };

    assert!(env
        .update::<Result<Nat, ApproveError>>(
            env.ekoke_id,
            alice(),
            "icrc2_approve",
            Encode!(&args).unwrap()
        )
        .is_err());

    // bad created at
    let args = ApproveArgs {
        spender: bob_account(),
        from_subaccount: None,
        amount: 100u64.into(),
        expected_allowance: None,
        expires_at: None,
        fee: None,
        memo: None,
        created_at_time: Some(0),
    };

    assert!(env
        .update::<Result<Nat, ApproveError>>(
            env.ekoke_id,
            alice(),
            "icrc2_approve",
            Encode!(&args).unwrap()
        )
        .is_err());

    // bad expiration
    let args = ApproveArgs {
        spender: bob_account(),
        from_subaccount: None,
        amount: 100u64.into(),
        expected_allowance: None,
        expires_at: Some(0),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    assert!(env
        .update::<Result<Nat, ApproveError>>(
            env.ekoke_id,
            alice(),
            "icrc2_approve",
            Encode!(&args).unwrap()
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_icrc2_transfer_from() {
    let env = TestEnv::init();

    // approve admin to spend
    let approve_args = ApproveArgs {
        spender: admin().into(),
        from_subaccount: None,
        amount: 100u64.into(),
        expected_allowance: None,
        expires_at: None,
        fee: None,
        memo: None,
        created_at_time: None,
    };
    assert!(env
        .update::<Result<Nat, ApproveError>>(
            env.ekoke_id,
            alice(),
            "icrc2_approve",
            Encode!(&approve_args).unwrap()
        )
        .is_ok());

    // inspect icrc2_transfer_from
    let args = TransferFromArgs {
        from: alice_account(),
        spender_subaccount: None,
        to: bob_account(),
        amount: 50u64.into(),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    assert!(env
        .update::<Result<Nat, TransferFromError>>(
            env.ekoke_id,
            admin(),
            "icrc2_transfer_from",
            Encode!(&args).unwrap()
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_fail_inspect_icrc2_transfer_from() {
    let env = TestEnv::init();

    // approve admin to spend
    let approve_args = ApproveArgs {
        spender: admin().into(),
        from_subaccount: None,
        amount: 100u64.into(),
        expected_allowance: None,
        expires_at: None,
        fee: None,
        memo: None,
        created_at_time: None,
    };
    assert!(env
        .update::<Result<Nat, ApproveError>>(
            env.ekoke_id,
            alice(),
            "icrc2_approve",
            Encode!(&approve_args).unwrap()
        )
        .is_ok());

    // bad fee
    let args = TransferFromArgs {
        from: alice_account(),
        spender_subaccount: None,
        to: bob_account(),
        amount: 50u64.into(),
        fee: Some(1u64.into()),
        memo: None,
        created_at_time: None,
    };

    assert!(env
        .update::<Result<Nat, TransferFromError>>(
            env.ekoke_id,
            admin(),
            "icrc2_transfer_from",
            Encode!(&args).unwrap()
        )
        .is_err());

    // bad memo
    let args = TransferFromArgs {
        from: alice_account(),
        spender_subaccount: None,
        to: bob_account(),
        amount: 50u64.into(),
        fee: None,
        memo: Some(vec![0x00, 0x01].into()),
        created_at_time: None,
    };

    assert!(env
        .update::<Result<Nat, TransferFromError>>(
            env.ekoke_id,
            admin(),
            "icrc2_transfer_from",
            Encode!(&args).unwrap()
        )
        .is_err());

    // bad created at
    let args = TransferFromArgs {
        from: alice_account(),
        spender_subaccount: None,
        to: bob_account(),
        amount: 50u64.into(),
        fee: None,
        memo: None,
        created_at_time: Some(0),
    };

    assert!(env
        .update::<Result<Nat, TransferFromError>>(
            env.ekoke_id,
            admin(),
            "icrc2_transfer_from",
            Encode!(&args).unwrap()
        )
        .is_err());
}
