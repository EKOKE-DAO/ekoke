use candid::Nat;
use did::dilazionato::{DilazionatoResult, Token};
use did::ID;

use super::configuration::Configuration;
use super::storage::ContractStorage;
use crate::client::{fly_client, FlyClient};
use crate::utils::caller;

pub struct Minter;

impl Minter {
    pub async fn mint(
        contract_id: &ID,
        installments: u64,
        contract_value: u64,
    ) -> DilazionatoResult<(Vec<Token>, Vec<Nat>)> {
        // get reward for contract
        let mfly_reward = fly_client(Configuration::get_fly_canister())
            .get_contract_reward(contract_id.clone(), installments)
            .await?;

        // make tokens
        let next_token_id = ContractStorage::total_supply();
        let mut tokens = Vec::with_capacity(installments as usize);
        let mut tokens_ids = Vec::with_capacity(installments as usize);
        let token_value: u64 = contract_value / installments;

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
                operator: None,
                owner: None,
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
