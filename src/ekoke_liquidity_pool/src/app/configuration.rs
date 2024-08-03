//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{ICP_LEDGER_CANISTER_MEMORY_ID, MEMORY_MANAGER};

thread_local! {
    /// ICP ledger canister
    static ICP_LEDGER_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(ICP_LEDGER_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );
}

/// canister configuration
pub struct Configuration;

impl Configuration {
    /// Set icp ledger canister principal
    pub fn set_icp_ledger_canister(canister_id: Principal) {
        ICP_LEDGER_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get icp ledger canister principal
    #[allow(dead_code)]
    pub fn get_icp_ledger_canister() -> Principal {
        ICP_LEDGER_CANISTER.with(|icp| icp.borrow().get().0)
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_set_icp_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_icp_ledger_canister(principal);
        assert_eq!(Configuration::get_icp_ledger_canister(), principal);
    }
}
