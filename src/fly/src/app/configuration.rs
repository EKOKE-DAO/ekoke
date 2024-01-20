//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::{StorableAccount, StorablePrincipal};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use icrc::icrc1::account::Account;

use crate::app::memory::{
    MEMORY_MANAGER, MINTING_ACCOUNT_MEMORY_ID, SWAP_ACCOUNT_MEMORY_ID, XRC_CANISTER_MEMORY_ID,
};

thread_local! {
    /// Minting account
    static MINTING_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(MINTING_ACCOUNT_MEMORY_ID)),
        Account {
            owner: Principal::anonymous(),
            subaccount: None
        }.into()).unwrap()
    );

    /// Swap account
    static SWAP_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(SWAP_ACCOUNT_MEMORY_ID)),
        Account {
            owner: Principal::anonymous(),
            subaccount: None
        }.into()).unwrap()
    );

    /// Swap account
    static XRC_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(XRC_CANISTER_MEMORY_ID)),
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

    /// Set swap account
    pub fn set_swap_account(swap_account: Account) {
        SWAP_ACCOUNT.with_borrow_mut(|cell| {
            cell.set(swap_account.into()).unwrap();
        });
    }

    /// Get minting account address
    pub fn get_minting_account() -> Account {
        MINTING_ACCOUNT.with(|ma| ma.borrow().get().0)
    }

    /// Get swap account address
    pub fn get_swap_account() -> Account {
        SWAP_ACCOUNT.with(|sa| sa.borrow().get().0)
    }

    /// Set xrc canister address
    pub fn set_xrc_canister(canister_id: Principal) {
        XRC_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get xrc canister address
    #[allow(dead_code)]
    pub fn get_xrc_canister() -> Principal {
        XRC_CANISTER.with(|xrc| xrc.borrow().get().0)
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
    fn test_should_set_swap_account() {
        let swap_account = bob_account();
        Configuration::set_swap_account(swap_account);
        assert_eq!(Configuration::get_swap_account(), swap_account);
    }

    #[test]
    fn test_should_set_xrc_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_xrc_canister(principal);
        assert_eq!(Configuration::get_xrc_canister(), principal);
    }
}
