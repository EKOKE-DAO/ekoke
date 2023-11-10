use std::cell::RefCell;

use candid::Principal;
use did::dilazionato::{ConfigurationError, DilazionatoError, DilazionatoResult};
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, StableVec};

use crate::app::memory::{
    CANISTER_CUSTODIANS_MEMORY_ID, CREATED_AT_MEMORY_ID, FLY_CANISTER_MEMORY_ID, LOGO_MEMORY_ID,
    MARKETPLACE_CANISTER_MEMORY_ID, MEMORY_MANAGER, NAME_MEMORY_ID, SYMBOL_MEMORY_ID,
    UPGRADED_AT_MEMORY_ID,
};
use crate::constants::{DEFAULT_LOGO, DEFAULT_NAME, DEFAULT_SYMBOL};

thread_local! {
    /// Principals that can manage the canister
    static CANISTER_CUSTODIANS: RefCell<StableVec<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableVec::new(MEMORY_MANAGER.with(|mm| mm.get(CANISTER_CUSTODIANS_MEMORY_ID))).unwrap()
    );

    /// Fly Canister principal
    static FLY_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(FLY_CANISTER_MEMORY_ID)), Principal::anonymous().into()).unwrap()
    );

    /// Marketplace Canister principal
    static MARKETPLACE_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(MARKETPLACE_CANISTER_MEMORY_ID)), Principal::anonymous().into()).unwrap()
    );

    /// Contract logo
    static LOGO: RefCell<StableCell<Option<String>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(LOGO_MEMORY_ID)), Some(DEFAULT_LOGO.to_string())).unwrap()
    );

    /// Contract name
    static NAME: RefCell<StableCell<Option<String>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(NAME_MEMORY_ID)), Some(DEFAULT_NAME.to_string())).unwrap()
    );

    /// Contract symbol
    static SYMBOL: RefCell<StableCell<Option<String>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(SYMBOL_MEMORY_ID)), Some(DEFAULT_SYMBOL.to_string())).unwrap()
    );

    /// Contract creation timestamp
    static CREATED_AT: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(CREATED_AT_MEMORY_ID)), crate::utils::time()).unwrap()
    );

    /// Contract last upgrade timestamp
    static UPGRADED_AT: RefCell<StableCell<Option<u64>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(UPGRADED_AT_MEMORY_ID)), None).unwrap()
    );

}

pub struct Configuration;

impl Configuration {
    /// Get canisters custodians
    pub fn get_canister_custodians() -> Vec<Principal> {
        CANISTER_CUSTODIANS.with_borrow(|canister_custodians| {
            canister_custodians
                .iter()
                .map(|principal| *principal.as_principal())
                .collect()
        })
    }

    /// checks whether a principal is custodian
    pub fn is_custodian(caller: Principal) -> bool {
        Self::get_canister_custodians().contains(&caller)
    }

    /// Set canisters custodian
    pub fn set_canister_custodians(principals: &[Principal]) -> DilazionatoResult<()> {
        // check if custodians is empty
        if principals.is_empty() {
            return Err(DilazionatoError::Configuration(
                ConfigurationError::CustodialsCantBeEmpty,
            ));
        }

        // check if principal is anonymous
        if principals
            .iter()
            .any(|principal| principal == &Principal::anonymous())
        {
            return Err(DilazionatoError::Configuration(
                ConfigurationError::AnonymousCustodial,
            ));
        }

        CANISTER_CUSTODIANS.with_borrow_mut(|canister_custodians| {
            for _ in 0..canister_custodians.len() {
                canister_custodians.pop();
            }
            for principal in principals {
                canister_custodians
                    .push(&StorablePrincipal::from(*principal))
                    .map_err(|_| DilazionatoError::StorageError)?;
            }
            Ok(())
        })
    }

    pub fn get_logo() -> Option<String> {
        LOGO.with_borrow(|logo| logo.get().clone())
    }

    pub fn set_logo(logo: String) -> DilazionatoResult<()> {
        LOGO.with_borrow_mut(|cell| cell.set(Some(logo)))
            .map_err(|_| DilazionatoError::StorageError)?;

        Ok(())
    }

    pub fn get_name() -> Option<String> {
        NAME.with_borrow(|name| name.get().clone())
    }

    pub fn set_name(name: String) -> DilazionatoResult<()> {
        NAME.with_borrow_mut(|cell| cell.set(Some(name)))
            .map_err(|_| DilazionatoError::StorageError)?;

        Ok(())
    }

    pub fn get_symbol() -> Option<String> {
        SYMBOL.with_borrow(|logo| logo.get().clone())
    }

    pub fn set_symbol(symbol: String) -> DilazionatoResult<()> {
        SYMBOL
            .with_borrow_mut(|cell| cell.set(Some(symbol)))
            .map_err(|_| DilazionatoError::StorageError)?;

        Ok(())
    }

    pub fn get_created_at() -> u64 {
        CREATED_AT.with_borrow(|cell| *cell.get())
    }

    pub fn get_upgraded_at() -> u64 {
        UPGRADED_AT
            .with_borrow(|cell| *cell.get())
            .unwrap_or(Self::get_created_at())
    }

    pub fn set_upgraded_at() -> DilazionatoResult<()> {
        UPGRADED_AT
            .with_borrow_mut(|cell| cell.set(Some(crate::utils::time())))
            .map_err(|_| DilazionatoError::StorageError)?;

        Ok(())
    }

    pub fn get_fly_canister() -> Principal {
        FLY_CANISTER.with_borrow(|cell| *cell.get().as_principal())
    }

    pub fn set_fly_canister(canister: Principal) -> DilazionatoResult<()> {
        FLY_CANISTER
            .with_borrow_mut(|cell| cell.set(StorablePrincipal::from(canister)))
            .map_err(|_| DilazionatoError::StorageError)?;

        Ok(())
    }

    pub fn set_marketplace_canister(canister: Principal) -> DilazionatoResult<()> {
        MARKETPLACE_CANISTER
            .with_borrow_mut(|cell| cell.set(StorablePrincipal::from(canister)))
            .map_err(|_| DilazionatoError::StorageError)?;

        Ok(())
    }

    pub fn get_marketplace_canister() -> Principal {
        MARKETPLACE_CANISTER.with_borrow(|cell| *cell.get().as_principal())
    }
}

#[cfg(test)]
mod test {

    use std::time::Duration;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_set_and_get_canister_custodians() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        assert!(Configuration::get_canister_custodians().is_empty());
        assert!(Configuration::set_canister_custodians(&[principal]).is_ok());
        assert_eq!(Configuration::get_canister_custodians(), vec![principal,]);
    }

    #[test]
    fn test_should_reject_empty_custodians() {
        assert!(Configuration::set_canister_custodians(&[]).is_err());
    }

    #[test]
    fn test_should_reject_anonymous_custodians() {
        assert!(Configuration::set_canister_custodians(&[Principal::anonymous()]).is_err());
    }

    #[test]
    fn test_should_tell_if_custodian() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(Configuration::set_canister_custodians(&[principal]).is_ok());
        assert!(Configuration::is_custodian(principal));
        assert!(!Configuration::is_custodian(Principal::anonymous()));
    }

    #[test]
    fn test_should_get_and_set_logo() {
        assert_eq!(Configuration::get_logo().unwrap().as_str(), DEFAULT_LOGO);
        assert!(Configuration::set_logo("new logo".to_string()).is_ok());
        assert_eq!(Configuration::get_logo().unwrap().as_str(), "new logo");
    }

    #[test]
    fn test_should_get_and_set_name() {
        assert_eq!(Configuration::get_name().unwrap().as_str(), DEFAULT_NAME);
        assert!(Configuration::set_name("new name".to_string()).is_ok());
        assert_eq!(Configuration::get_name().unwrap().as_str(), "new name");
    }

    #[test]
    fn test_should_get_and_set_symbol() {
        assert_eq!(
            Configuration::get_symbol().unwrap().as_str(),
            DEFAULT_SYMBOL
        );
        assert!(Configuration::set_symbol("NFTT".to_string()).is_ok());
        assert_eq!(Configuration::get_symbol().unwrap().as_str(), "NFTT");
    }

    #[test]
    fn test_should_get_created_at() {
        assert!(Configuration::get_created_at() <= crate::utils::time());
    }

    #[test]
    fn test_should_get_and_set_upgraded_at() {
        let last_upgrade = Configuration::get_upgraded_at();
        assert!(Configuration::get_upgraded_at() <= crate::utils::time());
        std::thread::sleep(Duration::from_millis(100));
        assert!(Configuration::set_upgraded_at().is_ok());
        assert!(Configuration::get_upgraded_at() > last_upgrade);
    }

    #[test]
    fn test_should_get_and_set_fly_canister() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert_eq!(Configuration::get_fly_canister(), Principal::anonymous());
        assert!(Configuration::set_fly_canister(principal).is_ok());
        assert_eq!(Configuration::get_fly_canister(), principal);
    }

    #[test]
    fn test_should_get_and_set_marketplace_canister() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert_eq!(
            Configuration::get_marketplace_canister(),
            Principal::anonymous()
        );
        assert!(Configuration::set_marketplace_canister(principal).is_ok());
        assert_eq!(Configuration::get_marketplace_canister(), principal);
    }
}
