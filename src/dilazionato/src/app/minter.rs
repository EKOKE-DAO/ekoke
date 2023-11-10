use candid::{Nat, Principal};
use did::{
    dilazionato::{SellContractResult, Token},
    ID,
};

use crate::{client::FlyClient, utils::caller};

use super::{configuration::Configuration, storage::ContractStorage};

pub struct Minter;

impl Minter {
    pub async fn mint(
        contract_id: &ID,
        seller: Principal,
        installments: u64,
        contract_value: u64,
    ) -> SellContractResult<(Vec<Token>, Vec<Nat>)> {
        // get reward for contract
        let mfly_reward = FlyClient::from(Configuration::get_fly_canister())
            .get_contract_reward(contract_id.clone(), installments)
            .await?;

        // make tokens
        let next_token_id = ContractStorage::total_supply();
        let mut tokens = Vec::with_capacity(installments as usize);
        let mut tokens_ids = Vec::with_capacity(installments as usize);
        let token_value: u64 = contract_value / installments;
        let marketplace_canister = Configuration::get_marketplace_canister();

        for token_id in next_token_id..next_token_id + installments {
            tokens.push(Token {
                approved_at: Some(crate::utils::time()),
                approved_by: Some(caller()),
                burned_at: None,
                burned_by: None,
                contract_id: contract_id.clone(),
                id: token_id.into(),
                is_burned: false,
                minted_at: crate::utils::time(),
                minted_by: caller(),
                operator: Some(marketplace_canister), // * the operator must be the marketplace canister
                owner: Some(seller),
                transferred_at: None,
                transferred_by: None,
                mfly_reward,
                value: token_value,
            });
            tokens_ids.push(token_id.into());
        }

        Ok((tokens, tokens_ids))
    }
}
