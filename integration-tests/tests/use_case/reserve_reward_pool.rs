use candid::Nat;
use did::deferred::{ContractRegistration, ContractType, Seller};
use dip721::GenericValue;
use icrc::icrc1::account::Account;
use integration_tests::actor::{admin, alice};
use integration_tests::client::{DeferredClient, EkokeRewardPoolClient, IcrcLedgerClient};
use integration_tests::{ekoke_to_e8s, TestEnv};

#[test]
#[serial_test::serial]
fn test_should_reserve_a_reward_pool_on_ekoke() {
    let env = TestEnv::init();
    let deferred_client = DeferredClient::from(&env);
    let ekoke_reward_pool_client = EkokeRewardPoolClient::from(&env);
    let ekoke_ledger_client = IcrcLedgerClient::new(env.ekoke_ledger_id, &env);

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
        .register_contract(admin(), registration_data)
        .unwrap();
    assert_eq!(contract_id, 0_u64);

    // give allowance to reward canister to spend my ekoke'
    let reward_amount = ekoke_to_e8s(installments);
    let allowance_amount = reward_amount.clone() + 10_000u64;
    assert!(ekoke_ledger_client
        .icrc2_approve(
            alice(),
            Account::from(env.ekoke_reward_pool_id),
            allowance_amount,
            None,
        )
        .is_ok());

    // reserve pool
    assert!(ekoke_reward_pool_client
        .reserve_pool(
            Account {
                owner: alice(),
                subaccount: None,
            },
            contract_id.clone(),
            reward_amount // 1 ekoke for each NFT
        )
        .is_ok());

    // sign contract
    let res = deferred_client.admin_sign_contract(contract_id);
    assert!(res.is_ok());

    // verify reward
    let token = deferred_client.token_metadata(Nat::from(0_u64)).unwrap();
    let ekoke_reward = token
        .properties
        .iter()
        .find(|(k, _)| k == "token:ekoke_reward")
        .unwrap()
        .1
        .clone();
    assert_eq!(ekoke_reward, GenericValue::NatContent(ekoke_to_e8s(1)));
}
