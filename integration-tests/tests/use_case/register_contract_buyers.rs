use did::deferred::{Buyers, ContractRegistration, ContractType, Deposit, GenericValue, Seller};
use icrc::icrc1::account::Account;
use integration_tests::actor::{admin, alice, bob};
use integration_tests::client::DeferredClient;
use integration_tests::TestEnv;
use pretty_assertions::assert_eq;

#[test]
#[serial_test::serial]
fn test_as_seller_i_can_set_the_contract_buyers() {
    let env = TestEnv::init();
    let deferred_client = DeferredClient::from(&env);

    let registration_data = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            principal: alice(),
            quota: 100,
        }],
        buyers: Buyers {
            principals: vec![bob()],
            deposit_account: Account::from(alice()),
        },
        deposit: Deposit {
            value_fiat: 20_000,
            value_icp: 100,
        },
        value: 400_000,
        currency: "EUR".to_string(),
        installments: 400_000 / 100,
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("via roma 10".to_string()),
        )],
        restricted_properties: vec![],
        expiration: "2050-01-01".to_string(),
    };
    // approve deposit
    crate::helper::contract_deposit(
        &env,
        registration_data.buyers.deposit_account,
        registration_data.deposit.value_icp,
    );

    // call register
    let contract_id = deferred_client
        .register_contract(admin(), registration_data)
        .unwrap();

    // sign contract
    let res = deferred_client.sign_contract(contract_id.clone());
    assert!(res.is_ok());

    // update buyers
    assert!(deferred_client
        .update_contract_buyers(alice(), contract_id, vec![bob()])
        .is_ok());
    // get contract buyers
    let token = deferred_client.token_metadata(0_u64.into()).unwrap();
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
