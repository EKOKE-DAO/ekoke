use candid::Encode;
use integration_tests::actor::{admin, bob};
use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_should_inspect_is_admin() {
    let env = TestEnv::init();

    assert!(env
        .update::<()>(
            env.ekoke_erc20_swap_id,
            admin(),
            "admin_set_cketh_minter_canister",
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
            env.ekoke_erc20_swap_id,
            bob(),
            "admin_set_cketh_minter_canister",
            Encode!(&env.xrc_id).unwrap(),
        )
        .is_err());
}
