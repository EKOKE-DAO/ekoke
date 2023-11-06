use std::cell::RefCell;

use candid::Principal;
use did::sell_contract::{SellContractError, SellContractResult};
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{FLY_CANISTER_MEMORY_ID, MEMORY_MANAGER};

thread_local! {
    /// Fly token canister
    static FLY_TOKEN_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(FLY_CANISTER_MEMORY_ID)), Principal::anonymous().into()  ).unwrap()

    );
}

pub struct Configuration;

impl Configuration {
    /// Get fly token from configuration, if set
    pub fn get_fly_token_canister() -> Option<Principal> {
        let principal = FLY_TOKEN_CANISTER
            .with_borrow(|fly_token_canister| fly_token_canister.get().as_principal().clone());

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
}
