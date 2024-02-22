//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::{StorableAccount, StorablePrincipal};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use icrc::icrc1::account::Account;

use crate::app::memory::{ARCHIVE_CANISTER_MEMORY_ID, MEMORY_MANAGER, MINTING_ACCOUNT_MEMORY_ID};

thread_local! {
    /// Minting account
    static MINTING_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(MINTING_ACCOUNT_MEMORY_ID)),
        Account {
            owner: Principal::anonymous(),
            subaccount: None
        }.into()).unwrap()
    );



    /// Archive canister
    static ARCHIVE_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(ARCHIVE_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );
}

/// canister configuration
pub struct Configuration;

impl Configuration {
    /// Set minting account
    pub fn set_minting_account(minting_account: Account) {
        MINTING_ACCOUNT.with_borrow_mut(|cell| {
            cell.set(minting_account.into()).unwrap();
        });
    }

    /// Get minting account address
    pub fn get_minting_account() -> Account {
        MINTING_ACCOUNT.with(|ma| ma.borrow().get().0)
    }

    /// Set archive canister principal
    pub fn set_archive_canister(canister_id: Principal) {
        ARCHIVE_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get archive canister principal
    #[allow(dead_code)]
    pub fn get_archive_canister() -> Principal {
        ARCHIVE_CANISTER.with(|icp| icp.borrow().get().0)
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::bob_account;

    #[test]
    fn test_should_set_minting_account() {
        let minting_account = bob_account();
        Configuration::set_minting_account(minting_account);
        assert_eq!(Configuration::get_minting_account(), minting_account);
    }

    #[test]
    fn test_should_set_archive_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_archive_canister(principal);
        assert_eq!(Configuration::get_archive_canister(), principal);
    }
}
