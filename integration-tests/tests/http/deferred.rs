use did::deferred::{Agency, Contract, ContractRegistration, ContractType, Seller, TokenInfo};
use did::ID;
use dip721::GenericValue;
use integration_tests::actor::{admin, alice, bob};
use integration_tests::client::{DeferredClient, HttpClient};
use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_should_get_contracts() {
    let env = TestEnv::init();
    let contract_id = init_contract(&env);

    let http_client = HttpClient::new(env.deferred_id, &env);
    let contracts: Vec<ID> = http_client.http_request("getContracts", serde_json::json!({}));

    assert_eq!(contracts.len(), 1);
    assert_eq!(contracts[0], contract_id);

    // get contract by id

    let contract: serde_json::Value = http_client.http_request(
        "getContract",
        serde_json::json!({
            "id": contract_id,
        }),
    );
    // get contract
    let contract: Option<Contract> = serde_json::from_value(
        contract
            .as_object()
            .unwrap()
            .get("contract")
            .unwrap()
            .clone(),
    )
    .unwrap();
    assert_eq!(contract.unwrap().id, contract_id);

    // get unexisting contract
    let contract: serde_json::Value = http_client.http_request(
        "getContract",
        serde_json::json!({
            "id": 5000_u64,
        }),
    );
    // get contract
    let contract: Option<Contract> = serde_json::from_value(
        contract
            .as_object()
            .unwrap()
            .get("contract")
            .unwrap()
            .clone(),
    )
    .unwrap();
    assert!(contract.is_none());
}

#[test]
#[serial_test::serial]
fn test_should_get_token() {
    let env = TestEnv::init();
    let contract_id = init_contract(&env);

    let http_client = HttpClient::new(env.deferred_id, &env);
    let token: serde_json::Value = http_client.http_request(
        "getToken",
        serde_json::json!({
            "id": 1,
        }),
    );
    let token_info: Option<TokenInfo> =
        serde_json::from_value(token.as_object().unwrap().get("token").unwrap().clone()).unwrap();
    assert!(token_info.is_some());

    let token_info = token_info.unwrap();
    assert_eq!(token_info.token.id, 1u64);
    assert_eq!(token_info.token.contract_id, contract_id);

    // get unexisting token
    let token: serde_json::Value = http_client.http_request(
        "getToken",
        serde_json::json!({
            "id": 5000_u64,
        }),
    );

    let token_info: Option<TokenInfo> =
        serde_json::from_value(token.as_object().unwrap().get("token").unwrap().clone()).unwrap();
    assert!(token_info.is_none());
}

#[test]
#[serial_test::serial]
fn test_should_get_agencies() {
    let env = TestEnv::init();
    let deferred_client = DeferredClient::from(&env);

    let agency = Agency {
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
    };

    // give bob an agency
    deferred_client.admin_register_agency(bob(), agency.clone());

    let http_client = HttpClient::new(env.deferred_id, &env);
    let agencies: Vec<Agency> = http_client.http_request("getAgencies", serde_json::json!({}));

    assert_eq!(agencies.len(), 1);
    assert_eq!(agencies[0], agency);
}

fn init_contract(env: &TestEnv) -> ID {
    let deferred_client = DeferredClient::from(env);

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: bob(),
            quota: 100,
        }],
        buyers: vec![alice()],
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
        .register_contract(admin(), registration_data)
        .unwrap();
    assert_eq!(contract_id, 0_u64);

    // sign contract
    let res = deferred_client.admin_sign_contract(contract_id.clone());
    assert!(res.is_ok());

    contract_id
}
