use std::cell::RefCell;

use did::sell_contract::{Contract, MintError, SellContractError, SellContractResult, Token};
use did::ID;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};

use crate::app::memory::{CONTRACTS_MEMORY_ID, MEMORY_MANAGER, TOKENS_MEMORY_ID};

#[derive(Default)]
pub struct Storage;

thread_local! {

    /// Contracts storage (1 contract has many tokens)
    static CONTRACTS: RefCell<BTreeMap<ID, Contract, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(CONTRACTS_MEMORY_ID))));

    /// Tokens storage (NFTs)
    static TOKENS: RefCell<BTreeMap<ID, Token, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(TOKENS_MEMORY_ID))));
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
            return Err(SellContractError::Mint(MintError::ContractAlreadyExists(
                contract.id,
            )));
        }

        // check if tokens is empty
        if tokens.is_empty() || contract.tokens.is_empty() {
            return Err(SellContractError::Mint(MintError::ContractHasNoTokens));
        }

        // check if token mismatch
        if contract.tokens.len() != tokens.len() {
            return Err(SellContractError::Mint(MintError::TokensMismatch));
        }
        let mut contract_tokens: Vec<&ID> = tokens.iter().map(|t| &t.id).collect();
        let mut tokens_ids: Vec<&ID> = contract.tokens.iter().collect();
        contract_tokens.sort();
        tokens_ids.sort();
        if contract_tokens != tokens_ids {
            return Err(SellContractError::Mint(MintError::TokensMismatch));
        }

        TOKENS.with_borrow_mut(|tokens_storage| {
            for token in tokens {
                // check if token already exists
                if tokens_storage.contains_key(&token.id) {
                    return Err(SellContractError::Mint(MintError::TokenAlreadyExists(
                        token.id,
                    )));
                }
                // check if token is associated to the contract
                if token.contract_id != contract.id {
                    return Err(SellContractError::Mint(
                        MintError::TokenDoesNotBelongToContract(token.id),
                    ));
                }
                // check if token owner is the seller
                if token.owner != contract.seller {
                    return Err(SellContractError::Mint(MintError::BadMintTokenOwner(
                        token.id,
                    )));
                }

                tokens_storage.insert(token.id.clone(), token);
            }

            Ok(())
        })?;
        CONTRACTS.with_borrow_mut(|contracts| contracts.insert(contract.id.clone(), contract));

        Ok(())
    }

    /// Get token by id
    pub fn get_token(id: &ID) -> Option<Token> {
        TOKENS.with_borrow(|tokens| tokens.get(id).clone())
    }

    /// Get tokens by contract
    pub fn get_contract_tokens(contract_id: &ID) -> Option<Vec<Token>> {
        let token_ids = Self::get_contract(contract_id).map(|c| c.tokens)?;
        TOKENS.with_borrow(|tokens| {
            let mut contract_tokens: Vec<Token> = Vec::with_capacity(token_ids.len());
            for token_id in token_ids {
                if let Some(token) = tokens.get(&token_id) {
                    contract_tokens.push(token.clone());
                }
            }
            Some(contract_tokens)
        })
    }

    /// Lock token
    pub fn lock_token(token_id: &ID) -> SellContractResult<()> {
        TOKENS.with_borrow_mut(|tokens| {
            if let Some(mut token) = tokens.get(token_id) {
                token.locked = true;
                tokens.insert(token_id.clone(), token);
                Ok(())
            } else {
                Err(SellContractError::Mint(MintError::TokenNotFound(
                    token_id.clone(),
                )))
            }
        })
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
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let token_2 = Token {
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            fly_reward: 10,
            building: BuildingData {
                city: "Rome".to_string(),
                fiat_value: 250_000,
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
    }

    #[test]
    fn test_should_not_allow_duped_contract() {
        let seller =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        let contract_id = ID::random();
        let token_1 = Token {
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone()],
            expiration: "2040-06-01".to_string(),
            fly_reward: 10,
            building: BuildingData {
                city: "Rome".to_string(),
                fiat_value: 250_000,
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
            fly_reward: 10,
            building: BuildingData {
                city: "Rome".to_string(),
                fiat_value: 250_000,
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
        let token_id = ID::random();
        let token_1 = Token {
            id: token_id.clone(),
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let token_2 = Token {
            id: token_id,
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let contract = Contract {
            id: ID::random(),
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            fly_reward: 10,
            building: BuildingData {
                city: "Rome".to_string(),
                fiat_value: 250_000,
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
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let token_2 = Token {
            id: ID::random(),
            contract_id: ID::random(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let contract = Contract {
            id: ID::random(),
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            fly_reward: 10,
            building: BuildingData {
                city: "Rome".to_string(),
                fiat_value: 250_000,
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
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let token_2 = Token {
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: Principal::anonymous(),
            value: 100,
            locked: false,
        };
        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            fly_reward: 10,
            building: BuildingData {
                city: "Rome".to_string(),
                fiat_value: 250_000,
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
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let token_2 = Token {
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };
        let token_3 = Token {
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: seller,
            value: 100,
            locked: false,
        };

        let contract = Contract {
            id: contract_id,
            seller,
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone(), token_2.id.clone()],
            expiration: "2040-06-01".to_string(),
            fly_reward: 10,
            building: BuildingData {
                city: "Rome".to_string(),
                fiat_value: 250_000,
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
    fn test_should_lock_token() {
        let contract_id = ID::random();
        let token_1 = Token {
            id: ID::random(),
            contract_id: contract_id.clone(),
            owner: Principal::anonymous(),
            value: 100,
            locked: false,
        };
        let contract = Contract {
            id: contract_id,
            seller: Principal::anonymous(),
            buyers: vec![Principal::anonymous()],
            tokens: vec![token_1.id.clone()],
            expiration: "2040-06-01".to_string(),
            fly_reward: 10,
            building: BuildingData {
                city: "Rome".to_string(),
                fiat_value: 250_000,
            },
        };

        assert!(Storage::insert_contract(contract.clone(), vec![token_1.clone()]).is_ok());
        assert!(Storage::lock_token(&token_1.id).is_ok());
        assert_eq!(Storage::get_token(&token_1.id).unwrap().locked, true);
    }
}
