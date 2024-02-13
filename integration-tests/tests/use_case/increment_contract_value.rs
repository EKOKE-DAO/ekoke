use did::deferred::{ContractRegistration, ContractType, GenericValue, Seller};
use integration_tests::actor::alice;
use integration_tests::client::DeferredClient;
use integration_tests::TestEnv;
use pretty_assertions::assert_eq;

#[test]
#[serial_test::serial]
fn test_as_seller_i_can_set_the_contract_buyers() {
    let env = TestEnv::init(false);
    let deferred_client = DeferredClient::from(&env);

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
    let contract_id = deferred_client
        .register_contract(registration_data)
        .unwrap();

    // sign contract
    let res = deferred_client.admin_sign_contract(contract_id.clone());
    assert!(res.is_ok());

    // increment contract value
    assert!(deferred_client
        .seller_increment_contract_value(alice(), contract_id, 100_000, 1_000)
        .is_ok());

    // verify new value and supply
    assert_eq!(deferred_client.total_supply(), 5_000_u64);
}
