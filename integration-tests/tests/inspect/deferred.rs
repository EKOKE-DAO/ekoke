use candid::Encode;
use did::deferred::{ContractRegistration, ContractType, GenericValue, Seller};
use integration_tests::actor::{admin, alice, bob};
use integration_tests::client::DeferredClient;
use integration_tests::TestEnv;
use pretty_assertions::assert_eq;

#[test]
fn test_should_inspect_is_admin() {
    let env = TestEnv::init();

    assert!(env
        .update::<()>(
            env.deferred_id,
            admin(),
            "admin_set_fly_canister",
            Encode!(&env.marketplace_id).unwrap(),
        )
        .is_ok());

    // not an admin
    assert!(env
        .update::<()>(
            env.deferred_id,
            bob(),
            "admin_set_fly_canister",
            Encode!(&env.marketplace_id).unwrap(),
        )
        .is_err());
}

#[test]
fn test_should_inspect_is_custodian() {
    let env = TestEnv::init();
    let client = DeferredClient::new(&env);

    client.set_custodians(vec![alice(), bob()]);

    assert!(env
        .update::<()>(
            env.deferred_id,
            alice(),
            "set_name",
            Encode!(&"new name").unwrap(),
        )
        .is_ok());

    assert!(env
        .update::<()>(
            env.deferred_id,
            admin(),
            "set_name",
            Encode!(&"new name").unwrap(),
        )
        .is_err());
}
