//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::marketplace::MarketplaceResult;
use did::{StorableAccount, StorablePrincipal};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use icrc::icrc1::account::Account;

use crate::app::memory::{
    DEFERRED_CANISTER_MEMORY_ID, FLY_CANISTER_MEMORY_ID, FLY_LIQUIDITY_POOL_ACCOUNT_MEMORY_ID,
    ICP_LEDGER_CANISTER_MEMORY_ID, INTEREST_FOR_BUYER_MEMORY_ID, MEMORY_MANAGER,
    XRC_CANISTER_MEMORY_ID,
};
use crate::client::FlyClient;
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

    /// Fly liquidity pool account
    static FLY_LIQUIDITY_POOL_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(FLY_LIQUIDITY_POOL_ACCOUNT_MEMORY_ID)),
            Account {
                owner: Principal::anonymous(),
                subaccount: None,
            }.into()).unwrap()
    );

    /// Swap account
    static XRC_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(XRC_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// ICP ledger canister
    static ICP_LEDGER_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(ICP_LEDGER_CANISTER_MEMORY_ID)),
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

    /// Get fly liquidity pool account
    pub async fn get_fly_liquidity_pool_account() -> MarketplaceResult<Account> {
        let account = FLY_LIQUIDITY_POOL_ACCOUNT.with(|sa| sa.borrow().get().0);
        if account.owner == Principal::anonymous() {
            Self::update_fly_liquidity_pool_account().await
        } else {
            Ok(account)
        }
    }

    /// Update fly liquidity pool account
    pub async fn update_fly_liquidity_pool_account() -> MarketplaceResult<Account> {
        // call fly
        let liquidity_pool_account = FlyClient::from(Configuration::get_fly_canister())
            .liquidity_pool_accounts()
            .await?
            .icp;
        FLY_LIQUIDITY_POOL_ACCOUNT.with_borrow_mut(|cell| {
            cell.set(liquidity_pool_account.into()).unwrap();
        });
        Ok(liquidity_pool_account)
    }

    /// Set xrc canister address
    pub fn set_xrc_canister(canister_id: Principal) {
        XRC_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get xrc canister address
    pub fn get_xrc_canister() -> Principal {
        XRC_CANISTER.with(|xrc| xrc.borrow().get().0)
    }

    /// Set icp ledger canister address
    pub fn set_icp_ledger_canister(canister_id: Principal) {
        ICP_LEDGER_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get icp ledger canister address
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

    #[tokio::test]
    async fn test_should_get_fly_liquidity_pool_account() {
        assert_eq!(
            Configuration::get_fly_liquidity_pool_account()
                .await
                .unwrap(),
            Account::from(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap())
        )
    }

    #[test]
    fn test_should_set_xrc_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_xrc_canister(principal);
        assert_eq!(Configuration::get_xrc_canister(), principal);
    }

    #[test]
    fn test_should_set_icp_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_icp_ledger_canister(principal);
        assert_eq!(Configuration::get_icp_ledger_canister(), principal);
    }
}
