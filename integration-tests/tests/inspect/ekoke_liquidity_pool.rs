use candid::{Encode, Nat};
use integration_tests::actor::{admin, bob};
use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_should_inspect_is_admin() {
    let env = TestEnv::init();

    assert!(env
        .update::<Nat>(
            env.ekoke_liquidity_pool_id,
            admin(),
            "admin_cycles",
            Encode!().unwrap(),
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_fail_inspect_admin() {
    let env = TestEnv::init();
    // not an admin
    assert!(env
        .update::<Nat>(
            env.ekoke_liquidity_pool_id,
            bob(),
            "admin_cycles",
            Encode!().unwrap(),
        )
        .is_err());
}
