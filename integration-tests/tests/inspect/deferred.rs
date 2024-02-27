use candid::{Encode, Nat};
use did::deferred::{ContractRegistration, ContractType, DeferredResult, GenericValue, Seller};
use did::ID;
use dip721::NftError;
use integration_tests::actor::{admin, alice, bob};
use integration_tests::client::DeferredClient;
use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_should_inspect_is_admin() {
    let env = TestEnv::init();

    assert!(env
        .update::<()>(
            env.deferred_id,
            admin(),
            "admin_set_ekoke_ledger_canister",
            Encode!(&env.marketplace_id).unwrap(),
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_admin_not_admin() {
    let env = TestEnv::init();
    // not an admin
    assert!(env
        .update::<()>(
            env.deferred_id,
            bob(),
            "admin_set_ekoke_ledger_canister",
            Encode!(&env.marketplace_id).unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
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
}

#[test]
#[serial_test::serial]
fn test_should_inspect_is_custodian_not_custodian() {
    let env = TestEnv::init();
    let client = DeferredClient::new(&env);

    client.set_custodians(vec![alice(), bob()]);

    assert!(env
        .update::<()>(
            env.deferred_id,
            admin(),
            "set_name",
            Encode!(&"new name").unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_update_contract_property() {
    let env = TestEnv::init();
    let client = DeferredClient::new(&env);

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    // call register
    let contract_id = client
        .register_contract(admin(), registration_data)
        .unwrap();
    assert!(env
        .update::<DeferredResult<()>>(
            env.deferred_id,
            alice(),
            "update_contract_property",
            Encode!(
                &contract_id,
                &"contract:address",
                &GenericValue::TextContent("Via roma 123".to_string())
            )
            .unwrap(),
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_update_contract_property_unexisting_contract() {
    let env = TestEnv::init();

    assert!(env
        .update::<DeferredResult<()>>(
            env.deferred_id,
            alice(),
            "update_contract_property",
            Encode!(
                &Nat::from(0_u64),
                &"contract:address",
                &GenericValue::TextContent("Via roma 123".to_string())
            )
            .unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_update_contract_property_is_not_authorized() {
    let env = TestEnv::init();
    let client = DeferredClient::new(&env);

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    // call register
    let contract_id = client
        .register_contract(admin(), registration_data)
        .unwrap();
    assert!(env
        .update::<DeferredResult<()>>(
            env.deferred_id,
            bob(),
            "update_contract_property",
            Encode!(
                &contract_id,
                &"contract:address",
                &GenericValue::TextContent("Via roma 123".to_string())
            )
            .unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_update_contract_property_bad_key() {
    let env = TestEnv::init();
    let client = DeferredClient::new(&env);

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![
            Seller {
                principal: alice(),
                quota: 50,
            },
            Seller {
                principal: bob(),
                quota: 50,
            },
        ],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    // call register
    let contract_id = client
        .register_contract(admin(), registration_data)
        .unwrap();
    assert!(env
        .update::<DeferredResult<()>>(
            env.deferred_id,
            alice(),
            "update_contract_property",
            Encode!(
                &contract_id,
                &"token:address",
                &GenericValue::TextContent("Via roma 123".to_string())
            )
            .unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_update_contract_buyers() {
    let env = TestEnv::init();
    let client = DeferredClient::new(&env);

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    // call register
    let contract_id = client
        .register_contract(admin(), registration_data)
        .unwrap();
    assert!(env
        .update::<DeferredResult<()>>(
            env.deferred_id,
            alice(),
            "update_contract_buyers",
            Encode!(&contract_id, &vec![bob()]).unwrap(),
        )
        .is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_update_contract_buyers_unexisting_contract() {
    let env = TestEnv::init();

    // call register
    assert!(env
        .update::<DeferredResult<()>>(
            env.deferred_id,
            alice(),
            "update_contract_buyers",
            Encode!(&Nat::from(0_u64), &vec![bob()]).unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_update_contract_buyers_not_seller() {
    let env = TestEnv::init();
    let client = DeferredClient::new(&env);

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    // call register
    let contract_id = client
        .register_contract(admin(), registration_data)
        .unwrap();
    assert!(env
        .update::<DeferredResult<()>>(
            env.deferred_id,
            bob(),
            "update_contract_buyers",
            Encode!(&contract_id, &vec![bob()]).unwrap(),
        )
        .is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_register_contract() {
    let env = TestEnv::init();

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    let result: DeferredResult<ID> = env
        .update(
            env.deferred_id,
            admin(),
            "register_contract",
            Encode!(&registration_data).unwrap(),
        )
        .unwrap();

    assert!(result.is_ok());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_register_contract_unauthorized() {
    let env = TestEnv::init();

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    let result: anyhow::Result<DeferredResult<ID>> = env.update(
        env.deferred_id,
        alice(),
        "register_contract",
        Encode!(&registration_data).unwrap(),
    );

    assert!(result.is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_register_contract_no_sellers() {
    let env = TestEnv::init();

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    let result: anyhow::Result<DeferredResult<ID>> = env.update(
        env.deferred_id,
        admin(),
        "register_contract",
        Encode!(&registration_data).unwrap(),
    );

    assert!(result.is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_register_contract_installments_not_multiple() {
    let env = TestEnv::init();

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 13,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    let result: anyhow::Result<DeferredResult<ID>> = env.update(
        env.deferred_id,
        admin(),
        "register_contract",
        Encode!(&registration_data).unwrap(),
    );

    assert!(result.is_err());
}

#[test]
#[serial_test::serial]
fn test_should_inspect_burn() {
    let env = TestEnv::init();
    let client = DeferredClient::new(&env);

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: vec![bob()],
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    let contract_id = client
        .register_contract(admin(), registration_data)
        .unwrap();
    assert!(client.admin_sign_contract(contract_id.clone()).is_ok());

    // transfer token to buyer
    let token_id = Nat::from(1_u64);
    assert!(client
        .transfer_from(alice(), alice(), bob(), token_id.clone())
        .is_ok());

    // check burn
    assert!(env
        .update::<Result<Nat, NftError>>(
            env.deferred_id,
            bob(),
            "burn",
            Encode!(&token_id).unwrap(),
        )
        .is_ok());
}
