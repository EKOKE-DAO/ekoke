//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{
    INDEX_CANISTER_ID_MEMORY_ID, LEDGER_CANISTER_ID_MEMORY_ID, MEMORY_MANAGER,
};

thread_local! {
    /// Ledger canister
    static INDEX_CANISTER_ID: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(INDEX_CANISTER_ID_MEMORY_ID)),
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

    /// Set index canister id
    pub fn set_index_canister(canister_id: Principal) {
        INDEX_CANISTER_ID.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get index canister id
    #[allow(dead_code)]
    pub fn get_index_canister() -> Principal {
        INDEX_CANISTER_ID.with(|cell| cell.borrow().get().0)
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_set_ledger_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_ledger_canister(principal);
        assert_eq!(Configuration::get_ledger_canister(), principal);
    }

    #[test]
    fn test_should_set_index_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_index_canister(principal);
        assert_eq!(Configuration::get_index_canister(), principal);
    }
}
