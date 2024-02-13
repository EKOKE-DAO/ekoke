use candid::Nat;
use did::deferred::{ContractRegistration, ContractType, GenericValue, Seller};
use integration_tests::actor::{alice, bob};
use integration_tests::client::DeferredClient;
use integration_tests::TestEnv;
use pretty_assertions::assert_eq;

#[test]
#[serial_test::serial]
fn test_as_seller_i_can_register_a_sell_contract() {
    let env = TestEnv::init(false);
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
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    // call register
    let contract_id = deferred_client
        .register_contract(registration_data)
        .unwrap();
    assert_eq!(contract_id, 0_u64);

    // check unsigned contract and signed contracts
    let unsigned_contracts = deferred_client.admin_get_unsigned_contracts();
    assert_eq!(unsigned_contracts, vec![contract_id.clone()]);
    let signed_contract = deferred_client.get_signed_contracts();
    assert!(signed_contract.is_empty());

    // sign contract
    let res = deferred_client.admin_sign_contract(Nat::from(0_u64));
    assert!(res.is_ok());

    // check unsigned contract and signed contracts
    let unsigned_contracts = deferred_client.admin_get_unsigned_contracts();
    assert!(unsigned_contracts.is_empty());
    let signed_contract = deferred_client.get_signed_contracts();
    assert_eq!(signed_contract, vec![contract_id.clone()]);

    // verify contract tokens
    // there should be 400_000 / 100 = 4000 tokens
    let total_supply = deferred_client.total_supply();
    assert_eq!(total_supply, 4_000_u64);

    // first half for alice
    let token = deferred_client.token_metadata(Nat::from(0_u64)).unwrap();
    assert_eq!(token.is_burned, false);
    assert_eq!(token.owner.unwrap(), alice());
    assert_eq!(token.operator, Some(env.marketplace_id));
    let token_value = token
        .properties
        .iter()
        .find(|(k, _)| k == "token:value")
        .unwrap()
        .1
        .clone();
    assert_eq!(token_value, GenericValue::NatContent(100_u64.into()));

    let token = deferred_client.token_metadata(Nat::from(2000_u64)).unwrap();
    assert_eq!(token.owner.unwrap(), bob());
}
