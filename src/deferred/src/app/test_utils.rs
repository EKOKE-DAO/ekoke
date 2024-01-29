use candid::Principal;
use did::deferred::{Contract, Seller, Token};
use did::ID;
use dip721::TokenIdentifier;

use super::storage::ContractStorage;
use crate::utils::caller;

pub fn mock_token(id: u64, contract_id: u64) -> Token {
    Token {
        id: TokenIdentifier::from(id),
        contract_id: ID::from(contract_id),
        owner: Some(caller()),
        transferred_at: None,
        transferred_by: None,
        approved_at: None,
        approved_by: None,
        picoekoke_reward: 4000_u64.into(),
        burned_at: None,
        burned_by: None,
        minted_at: 0,
        value: 100,
        operator: None,
        is_burned: false,
        minted_by: Principal::anonymous(),
    }
}

pub fn mock_contract(id: u64, installments: u64) -> Contract {
    Contract {
        id: id.into(),
        r#type: did::deferred::ContractType::Financing,
        sellers: vec![Seller {
            principal: caller(),
            quota: 100,
        }],
        buyers: vec![Principal::management_canister()],
        tokens: vec![],
        installments,
        is_signed: false,
        initial_value: 250_000,
        value: 250_000,
        currency: "EUR".to_string(),
        properties: vec![(
            "contract:city".to_string(),
            dip721::GenericValue::TextContent("Rome".to_string()),
        )],
    }
}

pub fn store_mock_contract(token_ids: &[u64], contract_id: u64) {
    store_mock_contract_with(token_ids, contract_id, |_| {}, |_| {})
}

pub fn store_mock_contract_with<F, F2>(
    token_ids: &[u64],
    contract_id: u64,
    contract_fn: F,
    token_fn: F2,
) where
    F: FnOnce(&mut Contract),
    F2: FnOnce(&mut Token) + Copy,
{
    let mut tokens = Vec::new();
    for id in token_ids {
        let mut token = mock_token(*id, contract_id);
        token_fn(&mut token);
        tokens.push(token);
    }

    let mut contract = mock_contract(contract_id, token_ids.len() as u64);
    contract_fn(&mut contract);

    if let Err(err) = ContractStorage::insert_contract(contract) {
        panic!("{err}");
    }
    if let Err(err) = ContractStorage::sign_contract_and_mint_tokens(&contract_id.into(), tokens) {
        panic!("{err}");
    }
}

pub fn with_mock_token<F>(id: u64, contract_id: u64, f: F) -> Token
where
    F: FnOnce(&mut Token),
{
    let mut token = mock_token(id, contract_id);
    f(&mut token);
    token
}

pub fn with_mock_contract<F>(id: u64, installments: u64, f: F) -> Contract
where
    F: FnOnce(&mut Contract),
{
    let mut contract = mock_contract(id, installments);
    f(&mut contract);
    contract
}

pub fn alice() -> Principal {
    Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()
}
