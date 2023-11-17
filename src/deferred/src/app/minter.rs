use candid::{Nat, Principal};
use did::deferred::{DeferredResult, Token};
use did::ID;

use super::configuration::Configuration;
use super::storage::ContractStorage;
use crate::client::{fly_client, FlyClient};
use crate::utils::caller;

pub struct Minter;

impl Minter {
    pub async fn mint(
        contract_id: &ID,
        seller: Principal,
        installments: u64,
        contract_value: u64,
    ) -> DeferredResult<(Vec<Token>, Vec<Nat>)> {
        // get reward for contract
        let picofly_reward = fly_client(Configuration::get_fly_canister())
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
                picofly_reward: picofly_reward.clone(),
                value: token_value,
            });
            tokens_ids.push(token_id.into());
        }

        Ok((tokens, tokens_ids))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn test_should_mint_token() {
        let contract_id = ID::from(1);
        let seller = Principal::anonymous();
        let installments = 3;
        let contract_value = 120;

        let result = Minter::mint(&contract_id, seller, installments, contract_value).await;
        assert!(result.is_ok());
        let (tokens, tokens_ids) = result.unwrap();
        assert_eq!(tokens.len(), installments as usize);
        assert_eq!(tokens_ids.len(), installments as usize);
        assert_eq!(tokens[0].id, 0);
        assert_eq!(tokens[1].id, 1);
        assert_eq!(tokens[2].id, 2);
        assert_eq!(tokens[0].value, 40);
        assert_eq!(tokens[1].value, 40);
        assert_eq!(tokens[2].value, 40);
    }
}
