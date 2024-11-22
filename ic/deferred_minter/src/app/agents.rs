use std::cell::RefCell;

use candid::Principal;
use did::deferred::Agency;
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};

use crate::app::memory::{AGENCIES_MEMORY_ID, MEMORY_MANAGER};

thread_local! {
    /// Agencies storage (1 wallet has 1 agency)
    static AGENCIES: RefCell<BTreeMap<StorablePrincipal, Agency, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(AGENCIES_MEMORY_ID))));

}

pub struct Agents;

impl Agents {
    pub fn insert_agency(wallet: Principal, agency: Agency) {
        AGENCIES.with_borrow_mut(|agencies| {
            agencies.insert(wallet.into(), agency);
        })
    }

    pub fn get_agency_by_wallet(wallet: Principal) -> Option<Agency> {
        AGENCIES.with_borrow(|agencies| agencies.get(&StorablePrincipal::from(wallet)).clone())
    }

    /// Get all agencies
    pub fn get_agencies() -> Vec<Agency> {
        AGENCIES.with_borrow(|agencies| agencies.iter().map(|(_, agency)| agency.clone()).collect())
    }

    /// Remove agency by wallet
    pub fn remove_agency(wallet: Principal) {
        AGENCIES.with_borrow_mut(|agencies| {
            agencies.remove(&StorablePrincipal::from(wallet));
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::app::test_utils::{alice, mock_agency};

    #[test]
    fn test_should_store_and_retrieve_agency() {
        let wallet = alice();
        Agents::insert_agency(wallet, mock_agency());

        assert!(
            Agents::get_agency_by_wallet(wallet).is_some(),
            "Agency should be stored"
        );
        assert!(
            Agents::get_agency_by_wallet(Principal::anonymous()).is_none(),
            "Agency should not be stored"
        )
    }

    #[test]
    fn test_should_remove_agency() {
        let wallet = alice();
        Agents::insert_agency(wallet, mock_agency());
        Agents::remove_agency(wallet);

        assert!(
            Agents::get_agency_by_wallet(wallet).is_none(),
            "Agency should be removed"
        );
    }
}
