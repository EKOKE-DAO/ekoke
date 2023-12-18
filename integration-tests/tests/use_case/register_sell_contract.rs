use candid::{Encode, Nat};
use did::deferred::{ContractRegistration, ContractType, DeferredResult, GenericValue, Seller};
use did::ID;
use dip721::{NftError, TokenMetadata};
use integration_tests::actor::{admin, alice, bob};
use integration_tests::TestEnv;
use pretty_assertions::assert_eq;

#[test]
fn test_as_seller_i_can_register_a_sell_contract() {
    let env = TestEnv::init();

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
    let contract_id: DeferredResult<ID> = env
        .update(
            env.deferred_id,
            admin(),
            "register_contract",
            Encode!(&registration_data).unwrap(),
        )
        .unwrap();
    let contract_id = contract_id.unwrap();
    assert_eq!(contract_id, 0);

    // check unsigned contract and signed contracts
    let unsigned_contracts: Vec<ID> = env
        .query(
            env.deferred_id,
            admin(),
            "admin_get_unsigned_contracts",
            Encode!(&()).unwrap(),
        )
        .unwrap();
    assert_eq!(unsigned_contracts, vec![contract_id.clone()]);
    let signed_contract: Vec<ID> = env
        .query(
            env.deferred_id,
            admin(),
            "get_signed_contracts",
            Encode!(&()).unwrap(),
        )
        .unwrap();
    assert!(signed_contract.is_empty());

    // sign contract
    let res: DeferredResult<()> = env
        .update(
            env.deferred_id,
            admin(),
            "admin_sign_contract",
            Encode!(&contract_id).unwrap(),
        )
        .unwrap();
    assert!(res.is_ok());

    // check unsigned contract and signed contracts
    let unsigned_contracts: Vec<ID> = env
        .query(
            env.deferred_id,
            admin(),
            "admin_get_unsigned_contracts",
            Encode!(&()).unwrap(),
        )
        .unwrap();
    assert!(unsigned_contracts.is_empty());
    let signed_contract: Vec<ID> = env
        .query(
            env.deferred_id,
            admin(),
            "get_signed_contracts",
            Encode!(&()).unwrap(),
        )
        .unwrap();
    assert_eq!(signed_contract, vec![contract_id.clone()]);

    // verify contract tokens
    // there should be 400_000 / 100 = 4000 tokens
    let total_supply: Nat = env
        .query(
            env.deferred_id,
            admin(),
            "total_supply",
            Encode!(&()).unwrap(),
        )
        .unwrap();
    assert_eq!(total_supply, 4_000);

    // first half for alice
    let token: Result<TokenMetadata, NftError> = env
        .query(
            env.deferred_id,
            alice(),
            "token_metadata",
            Encode!(&Nat::from(0)).unwrap(),
        )
        .unwrap();

    let token = token.unwrap();
    assert_eq!(token.is_burned, false);
    assert_eq!(token.owner.unwrap(), alice());
    assert_eq!(token.operator, Some(env.marketplace_id));

    let token: Result<TokenMetadata, NftError> = env
        .query(
            env.deferred_id,
            alice(),
            "token_metadata",
            Encode!(&Nat::from(2000)).unwrap(),
        )
        .unwrap();

    let token = token.unwrap();
    assert_eq!(token.owner.unwrap(), bob());
}
