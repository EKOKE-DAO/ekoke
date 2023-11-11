//! # Inspect
//!
//! Dilazionato inspect message handler

use candid::{Nat, Principal};
use did::dilazionato::{DilazionatoError, DilazionatoResult, Token, TokenError};
use did::ID;
use dip721::NftError;
use itertools::Itertools;

use super::configuration::Configuration;
use super::storage::ContractStorage;

pub struct Inspect;

impl Inspect {
    /// Returns whether caller is custodian of the canister
    pub fn inspect_is_custodian(caller: Principal) -> bool {
        Configuration::is_custodian(caller)
    }

    /// Returns whether caller is owner or operator of the token
    pub fn inspect_is_owner_or_operator(
        caller: Principal,
        token_identifier: &Nat,
    ) -> Result<Token, NftError> {
        let token = match ContractStorage::get_token(token_identifier) {
            Some(token) => token,
            None => return Err(NftError::TokenNotFound),
        };

        let owner = match token.owner {
            Some(owner) => owner,
            None => return Err(NftError::UnauthorizedOwner),
        };

        if caller != owner && Some(caller) != token.operator {
            return Err(NftError::UnauthorizedOperator);
        }

        Ok(token)
    }

    /// Inspect burn, allow burn only if caller is owner or operator and token is owned by a buyer or a seller.
    pub fn inspect_burn(caller: Principal, token_identifier: &Nat) -> Result<(), NftError> {
        let token = match ContractStorage::get_token(token_identifier) {
            Some(token) => token,
            None => return Err(NftError::TokenNotFound),
        };
        let contract = match ContractStorage::get_contract(&token.contract_id) {
            Some(contract) => contract,
            None => return Err(NftError::TokenNotFound),
        };
        let owner = match token.owner {
            Some(owner) => owner,
            None => return Err(NftError::UnauthorizedOwner),
        };

        println!(
            "owner: {}, buyers {}, seller {}",
            owner,
            contract.buyers.iter().map(|x| x.to_string()).join(","),
            contract.seller
        );
        if !contract.buyers.contains(&owner) && owner != contract.seller {
            return Err(NftError::Other(
                "owner is not nor a buyer nor the seller".to_string(),
            ));
        }
        if caller != owner && Some(caller) != token.operator {
            return Err(NftError::UnauthorizedOperator);
        }

        Ok(())
    }

    /// Inspect register contract parameters:
    ///
    /// - caller must be custodian
    /// - contract must not exist
    /// - value must be multiple of installments
    /// - expiration must be in the future
    pub fn inspect_register_contract(
        caller: Principal,
        id: &ID,
        value: u64,
        installments: u64,
        expiration: &str,
    ) -> DilazionatoResult<()> {
        if !Self::inspect_is_custodian(caller) {
            return Err(DilazionatoError::Unauthorized);
        }
        // check if contract already exists
        if ContractStorage::get_contract(id).is_some() {
            return Err(DilazionatoError::Token(TokenError::ContractAlreadyExists(
                id.clone(),
            )));
        }

        // verify value must be multiple of installments
        if value % installments != 0 {
            return Err(DilazionatoError::Token(
                TokenError::ContractValueIsNotMultipleOfInstallments,
            ));
        }

        // check if expiration is YYYY-MM-DD and is not in the past
        match crate::utils::parse_date(expiration) {
            Ok(timestamp) if timestamp < crate::utils::time() => {
                return Err(DilazionatoError::Token(TokenError::InvalidExpirationDate));
            }
            Ok(_) => {}
            Err(_) => return Err(DilazionatoError::Token(TokenError::InvalidExpirationDate)),
        }

        Ok(())
    }

    pub fn inspect_is_seller(caller: Principal, contract: ID) -> DilazionatoResult<()> {
        let contract = match ContractStorage::get_contract(&contract) {
            Some(contract) => contract,
            None => {
                return Err(DilazionatoError::Token(TokenError::ContractNotFound(
                    contract,
                )))
            }
        };

        if contract.seller == caller {
            Ok(())
        } else {
            Err(DilazionatoError::Unauthorized)
        }
    }

    pub fn inspect_is_buyer(caller: Principal, contract: ID) -> DilazionatoResult<()> {
        let contract = match ContractStorage::get_contract(&contract) {
            Some(contract) => contract,
            None => {
                return Err(DilazionatoError::Token(TokenError::ContractNotFound(
                    contract,
                )))
            }
        };

        if contract.buyers.contains(&caller) {
            Ok(())
        } else {
            Err(DilazionatoError::Unauthorized)
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils;
    use crate::utils::caller;

    #[test]
    fn test_should_inspect_is_custodian() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_custodian(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert_eq!(Inspect::inspect_is_custodian(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert!(Configuration::set_canister_custodians(&[caller]).is_ok());
        assert_eq!(Inspect::inspect_is_custodian(caller), true);
    }

    #[test]
    fn test_should_is_owner_or_operator() {
        let caller = caller();
        test_utils::store_mock_contract_with(
            &[1],
            1,
            |_| {},
            |token| {
                token.owner = Some(caller);
                token.operator = None;
            },
        );
        assert!(Inspect::inspect_is_owner_or_operator(caller, &1.into()).is_ok());

        // with operator
        test_utils::store_mock_contract_with(
            &[2],
            2,
            |_| {},
            |token| {
                token.operator = Some(caller);
            },
        );
        assert!(ContractStorage::transfer(&2.into(), Principal::management_canister()).is_ok());
        assert!(Inspect::inspect_is_owner_or_operator(caller, &2.into()).is_ok());

        // no operator, no owner
        test_utils::store_mock_contract_with(
            &[3],
            3,
            |_| {},
            |token| {
                token.operator = Some(Principal::management_canister());
            },
        );
        assert!(ContractStorage::transfer(&3.into(), Principal::management_canister()).is_ok());
        assert!(Inspect::inspect_is_owner_or_operator(caller, &3.into()).is_err());
    }

    #[test]
    fn test_should_inspect_burn() {
        let caller = caller();
        // caller is owner and token is owned by buyer
        test_utils::store_mock_contract_with(
            &[1],
            1,
            |contract| {
                contract.seller = caller;
                contract.buyers = vec![Principal::management_canister()];
            },
            |token| {
                token.owner = Some(caller);
                token.operator = None;
            },
        );
        assert!(Inspect::inspect_burn(caller, &1.into()).is_ok());
        // caller is operator and token is owned by buyer
        test_utils::store_mock_contract_with(
            &[2],
            2,
            |contract| {
                contract.seller = caller;
                contract.buyers = vec![Principal::management_canister()];
            },
            |token| {
                token.operator = Some(caller);
            },
        );
        assert!(ContractStorage::transfer(&2.into(), Principal::management_canister()).is_ok());
        assert!(Inspect::inspect_burn(caller, &2.into()).is_ok());
        // caller is owner and token is owned by buyer
        test_utils::store_mock_contract_with(
            &[3],
            3,
            |contract| {
                contract.seller = Principal::management_canister();
                contract.buyers = vec![caller];
            },
            |token| {
                token.owner = Some(Principal::management_canister());
                token.operator = None;
            },
        );
        assert!(ContractStorage::transfer(&2.into(), caller).is_ok());
        assert!(Inspect::inspect_burn(caller, &1.into()).is_ok());
        // caller is operator and token is owned by buyer
        test_utils::store_mock_contract_with(
            &[4],
            4,
            |contract| {
                contract.seller = Principal::management_canister();
                contract.buyers = vec![caller];
            },
            |token| {
                token.owner = Some(Principal::management_canister());
                token.operator = Some(caller);
            },
        );
        assert!(ContractStorage::transfer(&4.into(), caller).is_ok());
        assert!(Inspect::inspect_burn(caller, &4.into()).is_ok());
        // caller is not owner nor operator
        test_utils::store_mock_contract_with(
            &[5],
            5,
            |contract| {
                contract.seller = caller;
                contract.buyers = vec![Principal::management_canister()];
            },
            |token| {
                token.operator = None;
            },
        );
        assert!(ContractStorage::transfer(&5.into(), Principal::management_canister()).is_ok());
        assert!(Inspect::inspect_burn(caller, &5.into()).is_err());
        // caller is owner, but owner is a third party
        test_utils::store_mock_contract_with(
            &[6],
            6,
            |contract| {
                contract.seller = Principal::management_canister();
                contract.buyers = vec![Principal::management_canister()];
            },
            |token| {
                token.owner = Some(Principal::management_canister());
                token.operator = None;
            },
        );
        assert!(ContractStorage::transfer(&6.into(), caller).is_ok());
        assert!(Inspect::inspect_burn(caller, &6.into()).is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_caller_is_not_custodian() {
        // caller is not custodian
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert!(Configuration::set_canister_custodians(&[crate::utils::caller()]).is_ok());
        assert!(
            Inspect::inspect_register_contract(caller, &1.into(), 100, 25, "2040-01-01").is_err()
        );
    }

    #[test]
    fn test_should_inspect_contract_register_contract_already_exists() {
        // contract already exists
        let caller = crate::utils::caller();
        assert!(Configuration::set_canister_custodians(&[caller]).is_ok());
        test_utils::store_mock_contract(&[1, 2], 2);
        assert!(
            Inspect::inspect_register_contract(caller, &2.into(), 100, 25, "2040-01-01").is_err()
        );
    }

    #[test]
    fn test_should_inspect_contract_register_value_is_not_multiple_of_installments() {
        let caller = crate::utils::caller();
        assert!(Configuration::set_canister_custodians(&[caller]).is_ok());
        assert!(
            Inspect::inspect_register_contract(caller, &1.into(), 110, 25, "2040-01-01").is_err()
        );
    }

    #[test]
    fn test_should_inspect_contract_register_invalid_expiration_date() {
        let caller = crate::utils::caller();
        assert!(Configuration::set_canister_custodians(&[caller]).is_ok());
        assert!(
            Inspect::inspect_register_contract(caller, &1.into(), 100, 25, "2020-01-01").is_err()
        );
    }

    #[test]
    fn test_should_inspect_contract_register() {
        let caller = crate::utils::caller();
        assert!(Configuration::set_canister_custodians(&[caller]).is_ok());
        assert!(
            Inspect::inspect_register_contract(caller, &1.into(), 100, 25, "2040-01-01").is_ok()
        );
    }

    #[test]
    fn test_should_inspect_caller_is_contract_seller() {
        let caller = crate::utils::caller();
        test_utils::store_mock_contract_with(
            &[6],
            1,
            |contract| {
                contract.seller = caller;
            },
            |token| {
                token.owner = Some(caller);
            },
        );
        assert!(Inspect::inspect_is_seller(caller, 1.into()).is_ok());
        assert!(Inspect::inspect_is_seller(Principal::management_canister(), 1.into()).is_err());
        // unexisting contract
        assert!(Inspect::inspect_is_seller(caller, 2.into()).is_err());
    }

    #[test]
    fn test_should_inspect_caller_is_contract_buyer() {
        let caller = crate::utils::caller();
        test_utils::store_mock_contract_with(
            &[6],
            1,
            |contract| {
                contract.buyers = vec![caller];
            },
            |token| {
                token.owner = Some(caller);
            },
        );
        assert!(Inspect::inspect_is_buyer(caller, 1.into()).is_ok());
        assert!(Inspect::inspect_is_buyer(Principal::management_canister(), 1.into()).is_err());
        // unexisting contract
        assert!(Inspect::inspect_is_buyer(caller, 2.into()).is_err());
    }
}
