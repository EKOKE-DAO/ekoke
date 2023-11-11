use candid::Principal;
use did::{
    dilazionato::{BuildingData, Contract, Token},
    ID,
};
use dip721::TokenIdentifier;

use crate::utils::caller;

use super::storage::ContractStorage;

pub fn mock_token(id: u64, contract_id: u64) -> Token {
    Token {
        id: TokenIdentifier::from(id),
        contract_id: ID::from(contract_id),
        owner: Some(caller()),
        transferred_at: None,
        transferred_by: None,
        approved_at: None,
        approved_by: None,
        mfly_reward: 4000,
        burned_at: None,
        burned_by: None,
        minted_at: 0,
        value: 100,
        operator: None,
        is_burned: false,
        minted_by: Principal::anonymous(),
    }
}

fn mock_contract(id: u64, token_ids: &[u64]) -> Contract {
    Contract {
        id: id.into(),
        seller: caller(),
        buyers: vec![Principal::management_canister()],
        tokens: token_ids
            .iter()
            .map(|id| TokenIdentifier::from(*id))
            .collect(),
        expiration: "2040-06-01".to_string(),
        initial_value: 250_000,
        value: 250_000,
        currency: "EUR".to_string(),
        building: BuildingData {
            city: "Rome".to_string(),
        },
    }
}

pub fn store_mock_contract(token_ids: &[u64], contract_id: u64) {
    store_mock_contract_with(token_ids, contract_id, |_| {}.into(), |_| {})
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

    let mut contract = mock_contract(contract_id, token_ids);
    contract_fn(&mut contract);

    if let Err(err) = ContractStorage::insert_contract(contract, tokens) {
        panic!("{err}");
    }
}
