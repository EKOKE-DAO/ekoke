use candid::Nat;
use did::deferred::{ContractRegistration, ContractType, Seller};
use dip721::GenericValue;
use icrc::icrc1::account::Account;
use integration_tests::actor::alice;
use integration_tests::client::{DeferredClient, FlyClient};
use integration_tests::{fly_to_picofly, TestEnv};

#[test]
fn test_should_reserve_a_reward_pool_on_fly() {
    let env = TestEnv::init();
    let deferred_client = DeferredClient::from(&env);
    let fly_client = FlyClient::from(&env);

    // register contract
    let installments = 400_000 / 100;
    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: vec![],
        value: 400_000,
        currency: "EUR".to_string(),
        installments,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
    };

    // call register
    let contract_id = deferred_client
        .register_contract(registration_data)
        .unwrap();
    assert_eq!(contract_id, 0);

    // reserve pool
    assert!(fly_client
        .reserve_pool(
            Account {
                owner: alice(),
                subaccount: None,
            },
            contract_id.clone(),
            fly_to_picofly(installments) // 1 fly for each NFT
        )
        .is_ok());

    // sign contract
    let res = deferred_client.admin_sign_contract(contract_id);
    assert!(res.is_ok());

    // verify reward
    let token = deferred_client.token_metadata(Nat::from(0)).unwrap();
    let picofly_reward = token
        .properties
        .iter()
        .find(|(k, _)| k == "token:picofly_reward")
        .unwrap()
        .1
        .clone();
    assert_eq!(picofly_reward, GenericValue::NatContent(fly_to_picofly(1)));
}
