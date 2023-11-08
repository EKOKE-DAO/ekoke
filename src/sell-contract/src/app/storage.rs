use std::cell::RefCell;

use candid::Principal;
use did::sell_contract::{
    Contract, SellContractError, SellContractResult, StorableTxEvent, Token, TokenError,
};
use did::{StorableNat, ID};
use dip721::TokenIdentifier;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};
use itertools::Itertools;

use crate::app::memory::{
    CONTRACTS_MEMORY_ID, MEMORY_MANAGER, TOKENS_MEMORY_ID, TRANSACTIONS_MEMORY_ID,
};

mod tx_history;
pub use tx_history::TxHistory;

#[derive(Default)]
pub struct Storage;

thread_local! {

    /// Contracts storage (1 contract has many tokens)
    static CONTRACTS: RefCell<BTreeMap<ID, Contract, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(CONTRACTS_MEMORY_ID))));

    /// Tokens storage (NFTs)
    static TOKENS: RefCell<BTreeMap<StorableNat, Token, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(TOKENS_MEMORY_ID))));

    /// Transactions history
    static TX_HISTORY: RefCell<BTreeMap<StorableNat, StorableTxEvent, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(TRANSACTIONS_MEMORY_ID))));
}

impl Storage {
    /// Get contract by id
    pub fn get_contract(id: &ID) -> Option<Contract> {
        CONTRACTS.with_borrow(|contracts| contracts.get(id).clone())
    }

    /// Insert contract
    pub fn insert_contract(contract: Contract, tokens: Vec<Token>) -> SellContractResult<()> {
        // check contract existance
        if Self::get_contract(&contract.id).is_some() {
            return Err(SellContractError::Token(TokenError::ContractAlreadyExists(
                contract.id,
            )));
        }

        // check if tokens is empty
        if tokens.is_empty() || contract.tokens.is_empty() {
            return Err(SellContractError::Token(TokenError::ContractHasNoTokens));
        }

        // check if token mismatch
        if contract.tokens.len() != tokens.len() {
            return Err(SellContractError::Token(TokenError::TokensMismatch));
        }
        let mut contract_tokens: Vec<&TokenIdentifier> = tokens.iter().map(|t| &t.id).collect();
        let mut tokens_ids: Vec<&TokenIdentifier> = contract.tokens.iter().collect();
        contract_tokens.sort();
        tokens_ids.sort();
        if contract_tokens != tokens_ids {
            return Err(SellContractError::Token(TokenError::TokensMismatch));
        }

        TOKENS.with_borrow_mut(|tokens_storage| {
            for token in tokens {
                // check if token already exists
                if tokens_storage.contains_key(&token.id.clone().into()) {
                    return Err(SellContractError::Token(TokenError::TokenAlreadyExists(
                        token.id,
                    )));
                }
                // check if token is associated to the contract
                if token.contract_id != contract.id {
                    return Err(SellContractError::Token(
                        TokenError::TokenDoesNotBelongToContract(token.id),
                    ));
                }
                // check if token owner is the seller
                if token.owner != Some(contract.seller) {
                    return Err(SellContractError::Token(TokenError::BadMintTokenOwner(
                        token.id,
                    )));
                }

                // register mint
                TxHistory::register_token_mint(&token);

                tokens_storage.insert(token.id.clone().into(), token);
            }

            Ok(())
        })?;
        CONTRACTS.with_borrow_mut(|contracts| contracts.insert(contract.id.clone(), contract));

        Ok(())
    }

    /// Get token by id
    pub fn get_token(id: &TokenIdentifier) -> Option<Token> {
        TOKENS.with_borrow(|tokens| tokens.get(&id.clone().into()).clone())
    }

    /// Get tokens by contract
    pub fn get_contract_tokens(contract_id: &ID) -> Option<Vec<Token>> {
        let token_ids = Self::get_contract(contract_id).map(|c| c.tokens)?;
        TOKENS.with_borrow(|tokens| {
            let mut contract_tokens: Vec<Token> = Vec::with_capacity(token_ids.len());
            for token_id in token_ids {
                if let Some(token) = tokens.get(&token_id.into()) {
                    contract_tokens.push(token.clone());
                }
            }
            Some(contract_tokens)
        })
    }

    /// Update the operator for all token to the new operator canister
    pub fn update_tokens_operator(operator: Principal) -> SellContractResult<()> {
        TOKENS.with_borrow_mut(|tokens| {
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
    pub fn burn_token(token_id: &TokenIdentifier) -> SellContractResult<()> {
        TOKENS.with_borrow_mut(|tokens| {
            if let Some(mut token) = tokens.get(&token_id.clone().into()) {
                // check if burned
                if token.is_burned {
                    return Err(SellContractError::Token(TokenError::TokenIsBurned(
                        token_id.clone(),
                    )));
                }
                token.is_burned = true;
                token.owner = None;
                token.burned_at = Some(crate::utils::time());
                token.burned_by = Some(crate::utils::caller());

                // register burn
                TxHistory::register_token_burn(&token);

                tokens.insert(token_id.clone().into(), token);
                Ok(())
            } else {
                Err(SellContractError::Token(TokenError::TokenNotFound(
                    token_id.clone(),
                )))
            }
        })
    }

    /// Transfer token to provided principal
    pub fn transfer(token_id: &TokenIdentifier, to: Principal) -> SellContractResult<()> {
        TOKENS.with_borrow_mut(|tokens| {
            if let Some(mut token) = tokens.get(&token_id.clone().into()) {
                // check if burned
                if token.is_burned {
                    return Err(SellContractError::Token(TokenError::TokenIsBurned(
                        token_id.clone(),
                    )));
                }
                token.owner = Some(to);
                token.transferred_at = Some(crate::utils::time());
                token.transferred_by = Some(crate::utils::caller());

                // register transfer
                TxHistory::register_transfer(&token);

                tokens.insert(token_id.clone().into(), token);
                Ok(())
            } else {
                Err(SellContractError::Token(TokenError::TokenNotFound(
                    token_id.clone(),
                )))
            }
        })
    }

    /// Returns the total amount of unique holders of tokens
    pub fn total_unique_holders() -> u64 {
        TOKENS.with_borrow(|tokens| {
            tokens
                .iter()
                .filter_map(|(_, token)| token.owner)
                .unique()
                .count()
        }) as u64
    }

    /// Get tokens owned by a certain principal
    pub fn tokens_by_owner(owner: Principal) -> Vec<TokenIdentifier> {
        TOKENS.with_borrow(|tokens| {
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
        TOKENS.with_borrow(|tokens| {
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
        TOKENS.with_borrow(|tokens| tokens.len())
    }
}

#[cfg(test)]
mod test {

    use candid::Principal;
    use did::sell_contract::BuildingData;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_insert_and_get_contract() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
            id: TokenIdentifier::from(2),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(Storage::get_contract(&contract.id).is_none());
        assert!(
            Storage::insert_contract(contract.clone(), vec![token_1.clone(), token_2.clone()])
                .is_ok()
        );
        assert!(Storage::get_contract(&contract.id).is_some());
        assert_eq!(Storage::get_contract_tokens(&contract.id).unwrap().len(), 2);
        assert!(Storage::get_token(&token_1.id).is_some());
        assert!(Storage::get_token(&token_2.id).is_some());
        assert_eq!(Storage::total_supply(), 2);
        assert_eq!(Storage::tokens_by_owner(seller).len(), 2);
        assert_eq!(Storage::tokens_by_operator(seller).len(), 1);
    }

    #[test]
    fn test_should_not_allow_duped_contract() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
            tokens: vec![token_1.id.clone()],
            expiration: "2040-06-01".to_string(),
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };
        assert!(Storage::insert_contract(contract.clone(), vec![token_1.clone()]).is_ok());
        assert!(Storage::insert_contract(contract, vec![token_1]).is_err());
    }

    #[test]
    fn test_should_not_allow_empty_contract() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![],
            expiration: "2040-06-01".to_string(),
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(Storage::insert_contract(contract.clone(), vec![]).is_err());
    }

    #[test]
    fn test_should_not_allow_duped_token() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let token_id = TokenIdentifier::from(1);
        let token_1 = Token {
            id: token_id.clone(),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
        let token_2 = Token {
            id: token_id,
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
            id: ID::random(),
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };
        assert!(Storage::insert_contract(contract, vec![token_1, token_2]).is_err());
    }

    #[test]
    fn test_should_not_allow_token_with_different_contract_id() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
        let token_2 = Token {
            id: TokenIdentifier::from(2),
            contract_id: ID::random(),
            owner: Some(seller),
            value: 100,
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
            id: ID::random(),
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };
        assert!(Storage::insert_contract(contract, vec![token_1, token_2]).is_err());
    }

    #[test]
    fn test_should_not_allow_token_owner_different_from_seller() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
        let token_2 = Token {
            id: TokenIdentifier::from(2),
            contract_id: contract_id.clone(),
            owner: Some(Principal::anonymous()),
            value: 100,
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
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };
        assert!(
            Storage::insert_contract(contract.clone(), vec![token_1.clone(), token_2.clone()])
                .is_err()
        );
    }

    #[test]
    fn test_should_not_allow_mismatching_tokens() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
        let token_2 = Token {
            id: TokenIdentifier::from(2),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
        let token_3 = Token {
            id: TokenIdentifier::from(3),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(Storage::insert_contract(
            contract.clone(),
            vec![token_1.clone(), token_2.clone(), token_3.clone()]
        )
        .is_err());
        assert!(
            Storage::insert_contract(contract.clone(), vec![token_1.clone(), token_3.clone()])
                .is_err()
        );
    }

    #[test]
    fn test_should_burn_token() {
        let contract_id = ID::random();
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(Principal::anonymous()),
            value: 100,
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
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(Storage::insert_contract(contract.clone(), vec![token_1.clone()]).is_ok());
        assert!(Storage::burn_token(&token_1.id).is_ok());
        assert_eq!(Storage::get_token(&token_1.id).unwrap().is_burned, true);
        assert!(Storage::get_token(&token_1.id).unwrap().burned_at.is_some());
        assert!(Storage::get_token(&token_1.id).unwrap().burned_by.is_some());
        assert!(Storage::get_token(&token_1.id).unwrap().owner.is_none());
    }

    #[test]
    fn test_should_transfer_token() {
        let contract_id = ID::random();
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(Principal::anonymous()),
            value: 100,
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
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        let new_owner =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        assert!(Storage::insert_contract(contract.clone(), vec![token_1.clone()]).is_ok());
        assert!(Storage::transfer(&token_1.id, new_owner).is_ok());
        assert_eq!(
            Storage::get_token(&token_1.id).unwrap().owner,
            Some(new_owner)
        );
        assert!(Storage::get_token(&token_1.id)
            .unwrap()
            .transferred_at
            .is_some());
        assert!(Storage::get_token(&token_1.id)
            .unwrap()
            .transferred_by
            .is_some());
    }

    #[test]
    fn test_should_return_unique_holders() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
        let token_2 = Token {
            id: TokenIdentifier::from(2),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(
            Storage::insert_contract(contract.clone(), vec![token_1.clone(), token_2.clone()])
                .is_ok()
        );
        assert_eq!(Storage::total_unique_holders(), 1);
    }

    #[test]
    fn test_should_update_token_operator() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let token_1 = Token {
            id: TokenIdentifier::from(1),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
            id: TokenIdentifier::from(2),
            contract_id: contract_id.clone(),
            owner: Some(seller),
            value: 100,
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
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };

        assert!(
            Storage::insert_contract(contract.clone(), vec![token_1.clone(), token_2.clone()])
                .is_ok()
        );
        assert!(Storage::update_tokens_operator(Principal::anonymous()).is_ok());
        assert_eq!(
            Storage::get_token(&token_1.id).unwrap().operator,
            Some(Principal::anonymous())
        );
    }
}
