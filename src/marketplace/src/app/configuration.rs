//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{DEFERRED_CANISTER_MEMORY_ID, FLY_CANISTER_MEMORY_ID, MEMORY_MANAGER};

thread_local! {
    /// Minting account
    static FLY_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(FLY_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// Swap account
    static DEFERRED_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(DEFERRED_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );
}

/// canister configuration
pub struct Configuration;

impl Configuration {
    /// Set minting account
    pub fn set_fly_canister(fly_canister: Principal) {
        FLY_CANISTER.with_borrow_mut(|cell| {
            cell.set(fly_canister.into()).unwrap();
        });
    }

    /// Set swap account
    pub fn set_deferred_canister(deferred_canister: Principal) {
        DEFERRED_CANISTER.with_borrow_mut(|cell| {
            cell.set(deferred_canister.into()).unwrap();
        });
    }

    /// Get minting account address
    pub fn get_fly_canister() -> Principal {
        FLY_CANISTER.with(|ma| ma.borrow().get().0)
    }

    /// Get swap account address
    pub fn get_deferred_canister() -> Principal {
        DEFERRED_CANISTER.with(|sa| sa.borrow().get().0)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::utils::id;

    #[test]
    fn test_should_set_deferred_canister() {
        let canister = id();
        Configuration::set_deferred_canister(canister);
        assert_eq!(Configuration::get_deferred_canister(), canister);
    }

    #[test]
    fn test_should_set_fly_canister() {
        let canister = id();
        Configuration::set_fly_canister(canister);
        assert_eq!(Configuration::get_fly_canister(), canister);
    }
}
