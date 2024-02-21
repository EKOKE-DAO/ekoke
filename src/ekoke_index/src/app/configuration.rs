//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{
    ARCHIVE_CANISTER_ID_MEMORY_ID, LEDGER_CANISTER_ID_MEMORY_ID, MEMORY_MANAGER,
};

thread_local! {

    /// Archive canister
    static ARCHIVE_CANISTER_ID: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(ARCHIVE_CANISTER_ID_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// Ledger canister
    static LEDGER_CANISTER_ID: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(LEDGER_CANISTER_ID_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );
}

/// canister configuration
pub struct Configuration;

impl Configuration {
    /// Set archive canister id
    pub fn set_archive_canister(canister_id: Principal) {
        ARCHIVE_CANISTER_ID.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get archive canister id
    pub fn get_archive_canister() -> Principal {
        ARCHIVE_CANISTER_ID.with(|cell| cell.borrow().get().0)
    }

    /// Set ledger canister id
    pub fn set_ledger_canister(canister_id: Principal) {
        LEDGER_CANISTER_ID.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get ledger canister id
    pub fn get_ledger_canister() -> Principal {
        LEDGER_CANISTER_ID.with(|cell| cell.borrow().get().0)
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_set_archive_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_archive_canister(principal);
        assert_eq!(Configuration::get_archive_canister(), principal);
    }

    #[test]
    fn test_should_set_ledger_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_ledger_canister(principal);
        assert_eq!(Configuration::get_ledger_canister(), principal);
    }
}
