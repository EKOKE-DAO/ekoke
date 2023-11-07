use std::cell::RefCell;

use candid::Principal;
use did::sell_contract::{ConfigurationError, SellContractError, SellContractResult};
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, StableVec};

use crate::app::memory::{CANISTER_CUSTODIANS_MEMORY_ID, FLY_CANISTER_MEMORY_ID, MEMORY_MANAGER};

thread_local! {
    /// Fly token canister
    static FLY_TOKEN_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(FLY_CANISTER_MEMORY_ID)), Principal::anonymous().into()  ).unwrap()
    );

    static CANISTER_CUSTODIANS: RefCell<StableVec<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableVec::new(MEMORY_MANAGER.with(|mm| mm.get(CANISTER_CUSTODIANS_MEMORY_ID))).unwrap()
    );
}

pub struct Configuration;

impl Configuration {
    /// Get fly token from configuration, if set
    pub fn get_fly_token_canister() -> Option<Principal> {
        let principal = FLY_TOKEN_CANISTER
            .with_borrow(|fly_token_canister| *fly_token_canister.get().as_principal());

        if principal == Principal::anonymous() {
            return None;
        }

        Some(principal)
    }

    /// Set fly token in configuration
    pub fn set_fly_token_canister(canister: Principal) -> SellContractResult<()> {
        FLY_TOKEN_CANISTER
            .with_borrow_mut(|fly_token_canister| fly_token_canister.set(canister.into()))
            .map_err(|_| SellContractError::StorageError)?;

        Ok(())
    }

    /// Get canisters custodians
    pub fn get_canister_custodians() -> Vec<Principal> {
        CANISTER_CUSTODIANS.with_borrow(|canister_custodians| {
            canister_custodians
                .iter()
                .map(|principal| *principal.as_principal())
                .collect()
        })
    }

    /// checks whether a principal is custodian
    pub fn is_custodian(caller: Principal) -> bool {
        Self::get_canister_custodians().contains(&caller)
    }

    /// Set canisters custodian
    pub fn set_canister_custodians(principals: &[Principal]) -> SellContractResult<()> {
        // check if custodians is empty
        if principals.is_empty() {
            return Err(SellContractError::Configuration(
                ConfigurationError::CustodialsCantBeEmpty,
            ));
        }

        // check if principal is anonymous
        if principals
            .iter()
            .any(|principal| principal == &Principal::anonymous())
        {
            return Err(SellContractError::Configuration(
                ConfigurationError::AnonymousCustodial,
            ));
        }

        CANISTER_CUSTODIANS.with_borrow_mut(|canister_custodians| {
            for _ in 0..canister_custodians.len() {
                canister_custodians.pop();
            }
            for principal in principals {
                canister_custodians
                    .push(&StorablePrincipal::from(*principal))
                    .map_err(|_| SellContractError::StorageError)?;
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_set_and_get_fly_canister() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        assert!(Configuration::get_fly_token_canister().is_none());
        assert!(Configuration::set_fly_token_canister(principal).is_ok());
        assert_eq!(Configuration::get_fly_token_canister().unwrap(), principal);
    }

    #[test]
    fn test_should_set_and_get_canister_custodians() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        assert!(Configuration::get_canister_custodians().is_empty());
        assert!(Configuration::set_canister_custodians(&[principal]).is_ok());
        assert_eq!(Configuration::get_canister_custodians(), vec![principal,]);
    }

    #[test]
    fn test_should_reject_empty_custodians() {
        assert!(Configuration::set_canister_custodians(&[]).is_err());
    }

    #[test]
    fn test_should_reject_anonymous_custodians() {
        assert!(Configuration::set_canister_custodians(&[Principal::anonymous()]).is_err());
    }

    #[test]
    fn test_should_tell_if_custodian() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(Configuration::set_canister_custodians(&[principal]).is_ok());
        assert!(Configuration::is_custodian(principal));
        assert!(!Configuration::is_custodian(Principal::anonymous()));
    }
}
