use did::deferred::{ContractRegistration, ContractType, GenericValue, Seller};
use integration_tests::actor::{alice, bob};
use integration_tests::client::DeferredClient;
use integration_tests::TestEnv;
use pretty_assertions::assert_eq;

#[test]
fn test_should_update_contract_property() {
    let env = TestEnv::init();
    let deferred_client = DeferredClient::from(&env);

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
        properties: vec![
            (
                "contract:address".to_string(),
                GenericValue::TextContent("via roma 10".to_string()),
            ),
            (
                "contract:architect".to_string(),
                GenericValue::TextContent("Gino Valle".to_string()),
            ),
        ],
    };

    // call register
    let contract_id = deferred_client
        .register_contract(registration_data)
        .unwrap();

    let res = deferred_client.admin_sign_contract(contract_id.clone());
    assert!(res.is_ok());

    // call update_contract_property
    assert!(deferred_client
        .update_contract_property(
            alice(),
            contract_id,
            "contract:architect".to_string(),
            GenericValue::TextContent("Renzo Piano".to_string())
        )
        .is_ok());

    let token_metadata = deferred_client.token_metadata(0.into()).unwrap();
    let value = token_metadata
        .properties
        .iter()
        .find(|(k, _)| k == "contract:architect")
        .unwrap();
    assert_eq!(
        value.1,
        GenericValue::TextContent("Renzo Piano".to_string())
    );
}
