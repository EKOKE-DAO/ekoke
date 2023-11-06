use std::cell::RefCell;

use candid::Principal;
use did::sell_contract::{ConfigurationError, SellContractError, SellContractResult};
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, StableVec};

use crate::app::memory::{CANISTER_CUSTODIALS_MEMORY_ID, FLY_CANISTER_MEMORY_ID, MEMORY_MANAGER};

thread_local! {
    /// Fly token canister
    static FLY_TOKEN_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(FLY_CANISTER_MEMORY_ID)), Principal::anonymous().into()  ).unwrap()
    );

    static CANISTER_CUSTODIALS: RefCell<StableVec<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableVec::new(MEMORY_MANAGER.with(|mm| mm.get(CANISTER_CUSTODIALS_MEMORY_ID))).unwrap()
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

    /// Get canisters custodials
    pub fn get_canister_custodials() -> Vec<Principal> {
        CANISTER_CUSTODIALS.with_borrow(|canister_custodials| {
            canister_custodials
                .iter()
                .map(|principal| *principal.as_principal())
                .collect()
        })
    }

    /// checks whether a principal is custodial
    pub fn is_custodial(caller: Principal) -> bool {
        Self::get_canister_custodials().contains(&caller)
    }

    /// Set canisters custodial
    pub fn set_canister_custodials(principals: &[Principal]) -> SellContractResult<()> {
        // check if custodials is empty
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

        CANISTER_CUSTODIALS.with_borrow_mut(|canister_custodials| {
            for _ in 0..canister_custodials.len() {
                canister_custodials.pop();
            }
            for principal in principals {
                canister_custodials
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
    fn test_should_set_and_get_canister_custodials() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        assert!(Configuration::get_canister_custodials().is_empty());
        assert!(Configuration::set_canister_custodials(&[principal]).is_ok());
        assert_eq!(Configuration::get_canister_custodials(), vec![principal,]);
    }

    #[test]
    fn test_should_reject_empty_custodials() {
        assert!(Configuration::set_canister_custodials(&[]).is_err());
    }

    #[test]
    fn test_should_reject_anonymous_custodials() {
        assert!(Configuration::set_canister_custodials(&[Principal::anonymous()]).is_err());
    }

    #[test]
    fn test_should_tell_if_custodial() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(Configuration::set_canister_custodials(&[principal]).is_ok());
        assert!(Configuration::is_custodial(principal));
        assert!(!Configuration::is_custodial(Principal::anonymous()));
    }
}
