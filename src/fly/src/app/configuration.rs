//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{MEMORY_MANAGER, MINTING_ACCOUNT_MEMORY_ID};

thread_local! {
    /// Minting account
    static MINTING_ACCOUNT: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(MINTING_ACCOUNT_MEMORY_ID)), Principal::anonymous().into()).unwrap()
    );
}

/// canister configuration
pub struct Configuration;

impl Configuration {
    /// Set minting account
    pub fn set_minting_account(minting_account: Principal) {
        MINTING_ACCOUNT.with(|ma| {
            ma.replace(
                StableCell::new(
                    MEMORY_MANAGER.with(|mm| mm.get(MINTING_ACCOUNT_MEMORY_ID)),
                    minting_account.into(),
                )
                .unwrap(),
            );
        });
    }

    /// Get minting account address
    pub fn get_minting_account() -> Principal {
        MINTING_ACCOUNT.with(|ma| ma.borrow().get().0)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_set_minting_account() {
        let minting_account = Principal::management_canister();
        Configuration::set_minting_account(minting_account);
        assert_eq!(Configuration::get_minting_account(), minting_account);
    }
}
