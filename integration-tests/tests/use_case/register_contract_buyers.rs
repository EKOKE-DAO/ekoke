use did::deferred::{ContractRegistration, ContractType, GenericValue, Seller};
use integration_tests::actor::{alice, bob};
use integration_tests::client::DeferredClient;
use integration_tests::TestEnv;
use pretty_assertions::assert_eq;

#[test]
fn test_as_seller_i_can_set_the_contract_buyers() {
    let env = TestEnv::init();
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

    // update buyers
    assert!(deferred_client
        .update_contract_buyers(alice(), contract_id, vec![bob()])
        .is_ok());
    // get contract buyers
    let token = deferred_client.token_metadata(0.into()).unwrap();
    let buyers = token
        .properties
        .iter()
        .find(|(k, _)| k == "contract:buyers")
        .unwrap()
        .1
        .clone();
    assert_eq!(
        buyers,
        GenericValue::NestedContent(vec![(
            "contract:buyer".to_string(),
            GenericValue::Principal(bob())
        )])
    );
}
