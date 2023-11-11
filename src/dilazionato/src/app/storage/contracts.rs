use candid::{Nat, Principal};
use did::dilazionato::{Contract, DilazionatoError, DilazionatoResult, Token, TokenError};
use did::{StorableNat, ID};
use dip721::TokenIdentifier;
use itertools::Itertools;

use super::{
    with_contract, with_contract_mut, with_contracts, with_contracts_mut, with_token,
    with_token_mut, with_tokens, with_tokens_mut, TxHistory,
};

pub struct ContractStorage;

impl ContractStorage {
    /// Get contract by id
    pub fn get_contract(id: &ID) -> Option<Contract> {
        with_contract(id, |contract| Ok(contract.clone())).ok()
    }

    /// Insert contract
    pub fn insert_contract(contract: Contract, tokens: Vec<Token>) -> DilazionatoResult<()> {
        // check contract existance
        if Self::get_contract(&contract.id).is_some() {
            return Err(DilazionatoError::Token(TokenError::ContractAlreadyExists(
                contract.id,
            )));
        }

        // check if tokens is empty
        if tokens.is_empty() || contract.tokens.is_empty() {
            return Err(DilazionatoError::Token(TokenError::ContractHasNoTokens));
        }

        // check if token mismatch
        if contract.tokens.len() != tokens.len() {
            return Err(DilazionatoError::Token(TokenError::TokensMismatch));
        }
        let mut contract_tokens: Vec<&TokenIdentifier> = tokens.iter().map(|t| &t.id).collect();
        let mut tokens_ids: Vec<&TokenIdentifier> = contract.tokens.iter().collect();
        contract_tokens.sort();
        tokens_ids.sort();
        if contract_tokens != tokens_ids {
            return Err(DilazionatoError::Token(TokenError::TokensMismatch));
        }

        Self::insert_contract_tokens(&contract.id, contract.seller, tokens)?;
        with_contracts_mut(|contracts| contracts.insert(contract.id.clone().into(), contract));

        Ok(())
    }

    /// Add provided tokens to a contract
    pub fn add_tokens_to_contract(contract_id: &ID, tokens: Vec<Token>) -> DilazionatoResult<()> {
        // check if tokens is empty
        if tokens.is_empty() {
            return Err(DilazionatoError::Token(TokenError::ContractHasNoTokens));
        }

        with_contract_mut(contract_id, |contract| {
            let new_value = contract.value + tokens.iter().map(|t| t.value).sum::<u64>();
            let token_ids = tokens
                .iter()
                .map(|t| t.id.clone())
                .collect::<Vec<TokenIdentifier>>();

            Self::insert_contract_tokens(contract_id, contract.seller, tokens)?;

            // update contract value and ids
            contract.value = new_value;
            contract.tokens.extend(token_ids);

            Ok(())
        })?;
        Ok(())
    }

    fn insert_contract_tokens(
        contract_id: &ID,
        seller: Principal,
        tokens: Vec<Token>,
    ) -> DilazionatoResult<()> {
        with_tokens_mut(|tokens_storage| {
            for token in tokens {
                // check if token already exists
                if tokens_storage.contains_key(&token.id.clone().into()) {
                    return Err(DilazionatoError::Token(TokenError::TokenAlreadyExists(
                        token.id,
                    )));
                }
                // check if token is associated to the contract
                if &token.contract_id != contract_id {
                    return Err(DilazionatoError::Token(
                        TokenError::TokenDoesNotBelongToContract(token.id),
                    ));
                }
                // check if token owner is the seller
                if token.owner != Some(seller) {
                    return Err(DilazionatoError::Token(TokenError::BadMintTokenOwner(
                        token.id,
                    )));
                }

                // register mint
                TxHistory::register_token_mint(&token);

                tokens_storage.insert(token.id.clone().into(), token);
            }

            Ok(())
        })
    }

    /// Get token by id
    pub fn get_token(id: &TokenIdentifier) -> Option<Token> {
        with_token(id, |token| Ok(token.clone())).ok()
    }

    /// get contracts
    pub fn get_contracts() -> Vec<ID> {
        with_contracts(|contracts| contracts.iter().map(|(key, _)| key.0.clone()).collect())
    }

    /// Update the contract  buyers
    pub fn update_contract_buyers(
        contract_id: &ID,
        buyers: Vec<Principal>,
    ) -> DilazionatoResult<()> {
        with_contract_mut(contract_id, |contract| {
            contract.buyers = buyers;
            Ok(())
        })
    }

    /// Update the operator for all token to the new operator canister
    pub fn update_tokens_operator(operator: Principal) -> DilazionatoResult<()> {
        with_tokens_mut(|tokens| {
            let new_tokens = tokens
                .iter()
                .map(|(id, token)| {
                    let mut token = token.clone();
                    token.operator = Some(operator);
                    (id.clone(), token)
                })
                .collect::<Vec<(StorableNat, Token)>>();
            for (id, token) in new_tokens {
                tokens.insert(id, token);
            }

            Ok(())
        })
    }

    /// Burn token
    pub fn burn_token(token_id: &TokenIdentifier) -> DilazionatoResult<Nat> {
        let (tx_id, token) = with_token_mut(token_id, |token| {
            // check if burned
            if token.is_burned {
                return Err(DilazionatoError::Token(TokenError::TokenIsBurned(
                    token_id.clone(),
                )));
            }
            token.is_burned = true;
            token.owner = None;
            token.burned_at = Some(crate::utils::time());
            token.burned_by = Some(crate::utils::caller());

            // register burn
            let tx_id = TxHistory::register_token_burn(token);

            Ok((tx_id, token.clone()))
        })?;

        // reduce contract value
        Self::reduce_contract_value_by(&token.contract_id, token.value)?;

        Ok(tx_id)
    }

    /// Reduce contract value by `decr_by`
    fn reduce_contract_value_by(contract_id: &ID, decr_by: u64) -> DilazionatoResult<()> {
        with_contract_mut(contract_id, |contract| {
            contract.value -= decr_by;

            Ok(())
        })
    }

    /// Transfer token to provided principal
    pub fn transfer(token_id: &TokenIdentifier, to: Principal) -> DilazionatoResult<Nat> {
        with_token_mut(token_id, |token| {
            // check if burned
            if token.is_burned {
                return Err(DilazionatoError::Token(TokenError::TokenIsBurned(
                    token_id.clone(),
                )));
            }
            token.owner = Some(to);
            token.transferred_at = Some(crate::utils::time());
            token.transferred_by = Some(crate::utils::caller());

            // register transfer
            let tx_id = TxHistory::register_transfer(token);

            Ok(tx_id)
        })
    }

    /// Returns the total amount of unique holders of tokens
    pub fn total_unique_holders() -> u64 {
        with_tokens(|tokens| {
            tokens
                .iter()
                .filter_map(|(_, token)| token.owner)
                .unique()
                .count()
        }) as u64
    }

    /// Get tokens owned by a certain principal
    pub fn tokens_by_owner(owner: Principal) -> Vec<TokenIdentifier> {
        with_tokens(|tokens| {
            tokens
                .iter()
                .filter_map(|(id, token)| {
                    if token.owner == Some(owner) {
                        Some(id.0.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
    }

    /// Get tokens with operator set to a certain principal
    pub fn tokens_by_operator(operator: Principal) -> Vec<TokenIdentifier> {
        with_tokens(|tokens| {
            tokens
                .iter()
                .filter_map(|(id, token)| {
                    if token.operator == Some(operator) {
                        Some(id.0.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
    }

    /// Returns the total supply of tokens
    pub fn total_supply() -> u64 {
        with_tokens(|tokens| tokens.len())
    }
}

#[cfg(test)]
mod test {

    use candid::Principal;
    use did::dilazionato::BuildingData;
    use pretty_assertions::assert_eq;

    use crate::app::test_utils::mock_token;

    use super::*;

    #[test]
    fn test_should_insert_and_get_contract() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let next_token_id = ContractStorage::total_supply();
        assert_eq!(next_token_id, Nat::from(0));
        let token_1 = Token {
            id: next_token_id.into(),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
            mfly_reward: 400,
            is_burned: false,
            transferred_at: None,
            transferred_by: None,
            approved_at: None,
            approved_by: None,
            burned_at: None,
            burned_by: None,
            minted_at: 0,
            minted_by: Principal::anonymous(),
            operator: Some(seller),
        };
        let token_2 = Token {
            id: (next_token_id + 1).into(),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
            mfly_reward: 400,
            is_burned: false,
            transferred_at: None,
            transferred_by: None,
            approved_at: None,
            approved_by: None,
            burned_at: None,
            burned_by: None,
            minted_at: 0,
            minted_by: Principal::anonymous(),
            operator: None,
        };
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(ContractStorage::get_contract(&contract.id).is_none());
        assert!(ContractStorage::insert_contract(
            contract.clone(),
            vec![token_1.clone(), token_2.clone()]
        )
        .is_ok());
        assert!(ContractStorage::get_contract(&contract.id).is_some());
        assert!(ContractStorage::get_token(&token_1.id).is_some());
        assert!(ContractStorage::get_token(&token_2.id).is_some());
        assert_eq!(ContractStorage::total_supply(), 2);
        assert_eq!(ContractStorage::tokens_by_owner(seller).len(), 2);
        assert_eq!(ContractStorage::tokens_by_operator(seller).len(), 1);
        assert_eq!(ContractStorage::get_contracts(), vec![contract.id]);
    }

    #[test]
    fn test_should_not_allow_duped_contract() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let token_1 = mock_token(1, 1);
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };
        assert!(ContractStorage::insert_contract(contract.clone(), vec![token_1.clone()]).is_ok());
        assert!(ContractStorage::insert_contract(contract, vec![token_1]).is_err());
    }

    #[test]
    fn test_should_not_allow_empty_contract() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(ContractStorage::insert_contract(contract.clone(), vec![]).is_err());
    }

    #[test]
    fn test_should_not_allow_duped_token() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let token_1 = mock_token(1, 1);
        let token_2 = mock_token(1, 1);
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };
        assert!(ContractStorage::insert_contract(contract, vec![token_1, token_2]).is_err());
    }

    #[test]
    fn test_should_not_allow_token_with_different_contract_id() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let token_1 = mock_token(1, 1);
        let token_2 = Token {
            id: TokenIdentifier::from(2),
            contract_id: ID::from(2),
            owner: Some(seller),
            value: 100,
            mfly_reward: 400,
            is_burned: false,
            transferred_at: None,
            transferred_by: None,
            approved_at: None,
            approved_by: None,
            burned_at: None,
            burned_by: None,
            minted_at: 0,
            minted_by: Principal::anonymous(),
            operator: None,
        };
        let contract = Contract {
            id: ID::from(1),
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };
        assert!(ContractStorage::insert_contract(contract, vec![token_1, token_2]).is_err());
    }

    #[test]
    fn test_should_not_allow_token_owner_different_from_seller() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let token_1 = mock_token(1, 1);
        let token_2 = Token {
            id: TokenIdentifier::from(2),
            contract_id: contract_id.clone(),
            owner: Some(Principal::anonymous()),
            value: 100,
            mfly_reward: 400,
            is_burned: false,
            transferred_at: None,
            transferred_by: None,
            approved_at: None,
            approved_by: None,
            burned_at: None,
            burned_by: None,
            minted_at: 0,
            minted_by: Principal::anonymous(),
            operator: None,
        };
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };
        assert!(ContractStorage::insert_contract(
            contract.clone(),
            vec![token_1.clone(), token_2.clone()]
        )
        .is_err());
    }

    #[test]
    fn test_should_not_allow_mismatching_tokens() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let token_1 = mock_token(1, 1);
        let token_2 = mock_token(2, 1);
        let token_3 = mock_token(3, 1);

        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(ContractStorage::insert_contract(
            contract.clone(),
            vec![token_1.clone(), token_2.clone(), token_3.clone()]
        )
        .is_err());
        assert!(ContractStorage::insert_contract(
            contract.clone(),
            vec![token_1.clone(), token_3.clone()]
        )
        .is_err());
    }

    #[test]
    fn test_should_burn_token() {
        let contract_id = ID::from(1);
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(Principal::anonymous()),
            value: 1000,
            mfly_reward: 4000,
            is_burned: false,
            transferred_at: None,
            transferred_by: None,
            approved_at: None,
            approved_by: None,
            burned_at: None,
            burned_by: None,
            minted_at: 0,
            minted_by: Principal::anonymous(),
            operator: None,
        };
        let contract = Contract {
            id: contract_id.clone(),
            seller: Principal::anonymous(),
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(ContractStorage::insert_contract(contract.clone(), vec![token_1.clone()]).is_ok());
        assert!(ContractStorage::burn_token(&token_1.id).is_ok());
        assert_eq!(
            ContractStorage::get_token(&token_1.id).unwrap().is_burned,
            true
        );
        assert!(ContractStorage::get_token(&token_1.id)
            .unwrap()
            .burned_at
            .is_some());
        assert!(ContractStorage::get_token(&token_1.id)
            .unwrap()
            .burned_by
            .is_some());
        assert!(ContractStorage::get_token(&token_1.id)
            .unwrap()
            .owner
            .is_none());
        // verify contract value has been decreased
        assert_eq!(
            ContractStorage::get_contract(&contract_id).unwrap().value,
            249_000
        );
        assert_eq!(
            ContractStorage::get_contract(&contract_id)
                .unwrap()
                .initial_value,
            250_000
        );
    }

    #[test]
    fn test_should_transfer_token() {
        let contract_id = ID::from(1);
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(Principal::anonymous()),
            value: 100,
            mfly_reward: 400,
            is_burned: false,
            transferred_at: None,
            transferred_by: None,
            approved_at: None,
            approved_by: None,
            burned_at: None,
            burned_by: None,
            minted_at: 0,
            minted_by: Principal::anonymous(),
            operator: None,
        };
        let contract = Contract {
            id: contract_id,
            seller: Principal::anonymous(),
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        let new_owner =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        assert!(ContractStorage::insert_contract(contract.clone(), vec![token_1.clone()]).is_ok());
        assert!(ContractStorage::transfer(&token_1.id, new_owner).is_ok());
        assert_eq!(
            ContractStorage::get_token(&token_1.id).unwrap().owner,
            Some(new_owner)
        );
        assert!(ContractStorage::get_token(&token_1.id)
            .unwrap()
            .transferred_at
            .is_some());
        assert!(ContractStorage::get_token(&token_1.id)
            .unwrap()
            .transferred_by
            .is_some());
    }

    #[test]
    fn test_should_return_unique_holders() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let token_1 = mock_token(1, 1);
        let token_2 = mock_token(2, 1);
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(ContractStorage::insert_contract(
            contract.clone(),
            vec![token_1.clone(), token_2.clone()]
        )
        .is_ok());
        assert_eq!(ContractStorage::total_unique_holders(), 1);
    }

    #[test]
    fn test_should_update_token_operator() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let token_1 = mock_token(1, 1);
        let token_2 = mock_token(2, 1);
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(ContractStorage::insert_contract(
            contract.clone(),
            vec![token_1.clone(), token_2.clone()]
        )
        .is_ok());
        assert!(ContractStorage::update_tokens_operator(Principal::anonymous()).is_ok());
        assert_eq!(
            ContractStorage::get_token(&token_1.id).unwrap().operator,
            Some(Principal::anonymous())
        );
    }

    #[test]
    fn test_should_update_contract_buyers() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let next_token_id = ContractStorage::total_supply();
        assert_eq!(next_token_id, Nat::from(0));
        let token_1 = mock_token(next_token_id, 1);
        let contract = Contract {
            id: contract_id.clone(),
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(ContractStorage::insert_contract(contract.clone(), vec![token_1.clone()]).is_ok());
        let buyer = seller;
        assert!(ContractStorage::update_contract_buyers(
            &contract_id,
            vec![Principal::anonymous(), buyer]
        )
        .is_ok());
        assert_eq!(
            ContractStorage::get_contract(&contract_id).unwrap().buyers,
            vec![Principal::anonymous(), buyer]
        );
    }

    #[test]
    fn test_should_increment_tokens() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::from(1);
        let next_token_id = ContractStorage::total_supply();
        assert_eq!(next_token_id, Nat::from(0));
        let token_1 = mock_token(next_token_id, 1);
        let contract = Contract {
            id: contract_id.clone(),
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone()],
            expiration: "2040-06-01".to_string(),
            initial_value: 100,
            value: 100,
            currency: "EUR".to_string(),
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(ContractStorage::insert_contract(contract.clone(), vec![token_1.clone()]).is_ok());
        assert_eq!(ContractStorage::total_supply(), 1);
        assert_eq!(ContractStorage::tokens_by_owner(seller).len(), 1);

        // create new tokens
        let token_2 = mock_token(next_token_id + 1, 1);
        assert!(ContractStorage::add_tokens_to_contract(&contract.id, vec![token_2]).is_ok());
        assert_eq!(ContractStorage::total_supply(), 2);
        assert_eq!(ContractStorage::tokens_by_owner(seller).len(), 2);
        assert_eq!(
            ContractStorage::get_contract(&contract_id).unwrap().value,
            200
        );
    }
}
