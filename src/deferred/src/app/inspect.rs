//! # Inspect
//!
//! Deferred inspect message handler

use candid::{Nat, Principal};
use did::deferred::{Contract, DeferredError, DeferredResult, Seller, Token, TokenError};
use did::ID;
use dip721::NftError;

use super::roles::RolesManager;
use super::storage::ContractStorage;

pub struct Inspect;

impl Inspect {
    /// Returns whether caller is custodian of the canister
    pub fn inspect_is_custodian(caller: Principal) -> bool {
        RolesManager::is_custodian(caller)
    }

    /// Returns whether caller is agent of the canister
    pub fn inspect_is_agent(caller: Principal) -> bool {
        RolesManager::is_agent(caller)
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

    /// Inspect whether the caller is owner or operator of the token and the token is not burned.
    pub fn inspect_transfer_from(
        caller: Principal,
        token_identifier: &Nat,
    ) -> Result<Token, NftError> {
        let token = Self::inspect_is_owner_or_operator(caller, token_identifier)?;
        if token.is_burned {
            return Err(NftError::ExistedNFT);
        }

        Ok(token)
    }

    /// Inspect burn, allow burn only if:
    /// - caller is owner or operator
    /// - token is owned by a buyer or a seller.
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

        if !contract.buyers.contains(&owner) && !contract.is_seller(&caller) {
            return Err(NftError::Other(
                "owner is not nor a buyer nor the seller".to_string(),
            ));
        }
        if caller != owner && Some(caller) != token.operator {
            return Err(NftError::UnauthorizedOperator);
        }

        Ok(())
    }

    /// Inspect update contract property:
    ///
    /// - caller must be one of custodian,seller,agent
    /// - contract must exist
    /// - key must start with "contract:"
    pub fn inspect_update_contract_property(
        caller: Principal,
        id: &ID,
        key: &str,
    ) -> DeferredResult<()> {
        if !key.starts_with("contract:") {
            return Err(DeferredError::Token(TokenError::BadContractProperty));
        }
        let contract = match ContractStorage::get_contract(id) {
            Some(contract) => contract,
            None => {
                return Err(DeferredError::Token(TokenError::ContractNotFound(
                    id.clone(),
                )))
            }
        };

        if !Self::inspect_is_custodian(caller)
            && !Self::inspect_is_agent(caller)
            && !contract.is_seller(&caller)
        {
            Err(DeferredError::Unauthorized)
        } else {
            Ok(())
        }
    }

    /// Inspect register contract parameters:
    ///
    /// - caller must be custodian or agent
    /// - value must be multiple of installments
    /// - must have sellers
    pub fn inspect_register_contract(
        caller: Principal,
        value: u64,
        sellers: &[Seller],
        installments: u64,
    ) -> DeferredResult<()> {
        if !Self::inspect_is_custodian(caller) && !Self::inspect_is_agent(caller) {
            return Err(DeferredError::Unauthorized);
        }

        if sellers
            .iter()
            .any(|seller| seller.principal == Principal::anonymous())
        {
            return Err(DeferredError::Token(TokenError::ContractHasNoSeller));
        }

        // verify value must be multiple of installments
        if value % installments != 0 {
            return Err(DeferredError::Token(
                TokenError::ContractValueIsNotMultipleOfInstallments,
            ));
        }

        let total_quota = sellers.iter().map(|seller| seller.quota).sum::<u8>();
        if total_quota != 100 {
            return Err(DeferredError::Token(
                TokenError::ContractSellerQuotaIsNot100,
            ));
        }

        Ok(())
    }

    pub fn inspect_is_seller(caller: Principal, contract: ID) -> DeferredResult<Contract> {
        let contract = match ContractStorage::get_contract(&contract) {
            Some(contract) => contract,
            None => return Err(DeferredError::Token(TokenError::ContractNotFound(contract))),
        };

        if contract.is_seller(&caller) {
            Ok(contract)
        } else {
            Err(DeferredError::Unauthorized)
        }
    }

    pub fn inspect_seller_increment_contract_value(
        caller: Principal,
        contract: ID,
    ) -> DeferredResult<Contract> {
        let contract = Self::inspect_is_seller(caller, contract)?;
        // check if is signed
        if !contract.is_signed {
            return Err(DeferredError::Token(TokenError::ContractNotSigned(
                contract.id,
            )));
        }

        Ok(contract)
    }
}

#[cfg(test)]
mod test {

    use did::deferred::{Role, Seller};
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{self, alice};
    use crate::utils::caller;

    #[test]
    fn test_should_inspect_is_custodian() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_custodian(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert_eq!(Inspect::inspect_is_custodian(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert_eq!(Inspect::inspect_is_custodian(caller), true);
    }

    #[test]
    fn test_should_inspect_is_agent() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_agent(caller), false);

        RolesManager::give_role(alice(), Role::Agent);
        assert_eq!(Inspect::inspect_is_agent(alice()), true);
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
    fn test_should_inspect_transfer_from() {
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
        assert!(Inspect::inspect_transfer_from(caller, &1.into()).is_ok());

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
        assert!(Inspect::inspect_transfer_from(caller, &2.into()).is_ok());

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
        assert!(Inspect::inspect_transfer_from(caller, &3.into()).is_err());

        test_utils::store_mock_contract_with(
            &[4],
            4,
            |_| {},
            |token| {
                token.owner = Some(caller);
                token.operator = None;
            },
        );
        assert!(ContractStorage::burn_token(&4.into()).is_ok());
        assert!(Inspect::inspect_transfer_from(caller, &4.into()).is_err());
    }

    #[test]
    fn test_should_inspect_burn() {
        let caller = caller();
        // caller is owner and token is owned by buyer
        test_utils::store_mock_contract_with(
            &[1],
            1,
            |contract| {
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
                contract.sellers = vec![Seller {
                    principal: Principal::management_canister(),
                    quota: 100,
                }];
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
                contract.sellers = vec![Seller {
                    principal: Principal::management_canister(),
                    quota: 100,
                }];
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
                contract.sellers = vec![Seller {
                    principal: Principal::management_canister(),
                    quota: 100,
                }];
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
        assert!(RolesManager::set_custodians(vec![crate::utils::caller()]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            100,
            &[Seller {
                principal: Principal::management_canister(),
                quota: 100,
            }],
            25,
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_value_is_not_multiple_of_installments() {
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            110,
            &[Seller {
                principal: Principal::management_canister(),
                quota: 100,
            }],
            25,
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_caller_is_not_agent() {
        // caller is not agent
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert!(Inspect::inspect_register_contract(
            caller,
            100,
            &[Seller {
                principal: Principal::management_canister(),
                quota: 100,
            }],
            25,
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_if_custodian() {
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            100,
            &[Seller {
                principal: Principal::management_canister(),
                quota: 100,
            }],
            25,
        )
        .is_ok());
    }

    #[test]
    fn test_should_inspect_contract_register_if_seller_is_anonymous() {
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            100,
            &[Seller {
                principal: Principal::anonymous(),
                quota: 100,
            }],
            25,
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_if_quota_is_not_100() {
        let caller = crate::utils::caller();
        RolesManager::give_role(caller, Role::Agent);
        assert!(Inspect::inspect_register_contract(
            caller,
            100,
            &[
                Seller {
                    principal: Principal::management_canister(),
                    quota: 20,
                },
                Seller {
                    principal: caller,
                    quota: 40,
                }
            ],
            25,
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_if_agent() {
        let caller = crate::utils::caller();
        RolesManager::give_role(caller, Role::Agent);
        assert!(Inspect::inspect_register_contract(
            caller,
            100,
            &[Seller {
                principal: Principal::management_canister(),
                quota: 100,
            }],
            25,
        )
        .is_ok());
    }

    #[test]
    fn test_should_inspect_caller_is_contract_seller() {
        let caller = crate::utils::caller();
        test_utils::store_mock_contract_with(
            &[6],
            1,
            |_| {},
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
    fn test_should_inspect_seller_increment_contract_value() {
        let caller = crate::utils::caller();
        let contract = test_utils::with_mock_contract(0, 1, |_| {});
        assert!(ContractStorage::insert_contract(contract).is_ok());
        let tokens = vec![test_utils::mock_token(0, 0)];
        assert!(Inspect::inspect_seller_increment_contract_value(caller, 0.into()).is_err());
        // sign contract
        assert!(ContractStorage::sign_contract_and_mint_tokens(&0.into(), tokens).is_ok());
        assert!(Inspect::inspect_seller_increment_contract_value(caller, 0.into()).is_ok());
        // not seller
        assert!(Inspect::inspect_seller_increment_contract_value(
            Principal::management_canister(),
            1.into()
        )
        .is_err());
        // unexisting contract
        assert!(Inspect::inspect_seller_increment_contract_value(caller, 2.into()).is_err());
    }

    #[test]
    fn test_should_inspect_update_contract_property() {
        let caller = crate::utils::caller();
        test_utils::store_mock_contract_with(
            &[6],
            1,
            |_| {},
            |token| {
                token.owner = Some(caller);
            },
        );
        assert!(
            Inspect::inspect_update_contract_property(caller, &1.into(), "contract:address")
                .is_ok()
        );
        assert!(Inspect::inspect_update_contract_property(caller, &1.into(), "foobar").is_err());
        assert!(Inspect::inspect_update_contract_property(
            Principal::management_canister(),
            &1.into(),
            "contract:address"
        )
        .is_err());
        // unexisting contract
        assert!(
            Inspect::inspect_update_contract_property(caller, &2.into(), "contract:address")
                .is_err()
        );
        // admin
        assert!(RolesManager::set_custodians(vec![Principal::management_canister()]).is_ok());
        assert!(Inspect::inspect_update_contract_property(
            Principal::management_canister(),
            &1.into(),
            "contract:address"
        )
        .is_ok());
    }
}
