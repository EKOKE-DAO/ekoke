use candid::Nat;
use did::deferred::{Agency, ContractRegistration, ContractType, GenericValue, Seller};
use integration_tests::actor::{alice, bob};
use integration_tests::client::DeferredClient;
use integration_tests::TestEnv;
use pretty_assertions::assert_eq;

#[test]
#[serial_test::serial]
fn test_should_register_agency_and_be_able_to_create_contract() {
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

    // give bob an agency
    deferred_client.admin_register_agency(
        bob(),
        Agency {
            name: "Bob's agency".to_string(),
            address: "Via Delle Botteghe Scure".to_string(),
            city: "Rome".to_string(),
            region: "Lazio".to_string(),
            zip_code: "00100".to_string(),
            country: "Italy".to_string(),
            continent: did::deferred::Continent::Europe,
            email: "email".to_string(),
            website: "website".to_string(),
            mobile: "mobile".to_string(),
            vat: "vat".to_string(),
            agent: "agent".to_string(),
            logo: None,
        },
    );

    // call register
    let contract_id = deferred_client
        .register_contract(bob(), registration_data.clone())
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

    // agency could remove himself
    assert!(deferred_client.remove_agency(bob()).is_ok());
}
