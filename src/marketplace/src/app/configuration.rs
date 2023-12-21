//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{
    DEFERRED_CANISTER_MEMORY_ID, FLY_CANISTER_MEMORY_ID, INTEREST_FOR_BUYER_MEMORY_ID,
    MEMORY_MANAGER,
};
use crate::constants::DEFAULT_INTEREST_MULTIPLIER_FOR_BUYER;

thread_local! {
    /// Fly canister
    static FLY_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(FLY_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// Deferred canister
    static DEFERRED_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(DEFERRED_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// Interest for buyer
    static INTEREST_RATE_FOR_BUYER: RefCell<StableCell<f64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(INTEREST_FOR_BUYER_MEMORY_ID)),
        DEFAULT_INTEREST_MULTIPLIER_FOR_BUYER).unwrap()
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

    /// Set interest rate for buyer
    pub fn set_interest_rate_for_buyer(interest_rate: f64) {
        INTEREST_RATE_FOR_BUYER.with_borrow_mut(|cell| {
            cell.set(interest_rate).unwrap();
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

    /// Get interest rate for buyer
    pub fn get_interest_rate_for_buyer() -> f64 {
        INTEREST_RATE_FOR_BUYER.with(|ir| *ir.borrow().get())
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

    #[test]
    fn test_should_set_interest_rate_for_buyer() {
        let interest_rate = 1.2;
        assert_eq!(
            Configuration::get_interest_rate_for_buyer(),
            DEFAULT_INTEREST_MULTIPLIER_FOR_BUYER
        );
        Configuration::set_interest_rate_for_buyer(interest_rate);
        assert_eq!(Configuration::get_interest_rate_for_buyer(), interest_rate);
    }
}
