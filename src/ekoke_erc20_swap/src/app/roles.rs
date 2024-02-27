//! Roles

use std::cell::RefCell;

use candid::Principal;
use did::ekoke::{ConfigurationError, EkokeError, EkokeResult};
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableVec};

use crate::app::memory::{ADMINS_MEMORY_ID, MEMORY_MANAGER};

thread_local! {
    /// Principals that can manage the canister
    static ADMINS: RefCell<StableVec<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableVec::new(MEMORY_MANAGER.with(|mm| mm.get(ADMINS_MEMORY_ID))).unwrap()
    );
}
pub struct RolesManager;

impl RolesManager {
    /// Returns whether principal is admin
    pub fn is_admin(principal: Principal) -> bool {
        ADMINS.with_borrow(|list| {
            for admin in list.iter() {
                if admin.0 == principal {
                    return true;
                }
            }
            false
        })
    }

    /// Get canister admins
    #[allow(dead_code)]
    pub fn get_admins() -> Vec<Principal> {
        ADMINS.with_borrow(|roles_map| roles_map.iter().map(|principal| principal.0).collect())
    }

    /// Set canister admins.
    ///
    /// WARNING: previous admins will be overwritten
    pub fn set_admins(admins: Vec<Principal>) -> EkokeResult<()> {
        // check if admins is empty
        if admins.is_empty() {
            return Err(EkokeError::Configuration(
                ConfigurationError::AdminsCantBeEmpty,
            ));
        }

        // check if principal is anonymous
        if admins
            .iter()
            .any(|principal| principal == &Principal::anonymous())
        {
            return Err(EkokeError::Configuration(
                ConfigurationError::AnonymousAdmin,
            ));
        }

        // Set admins
        ADMINS.with_borrow_mut(|admins_vec| {
            for _ in 0..admins_vec.len() {
                admins_vec.pop();
            }
            for admin in admins {
                admins_vec.push(&StorablePrincipal(admin)).unwrap();
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_set_and_get_canister_admins() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        assert!(RolesManager::get_admins().is_empty());
        assert!(RolesManager::set_admins(vec![principal]).is_ok());
        assert_eq!(RolesManager::get_admins(), vec![principal,]);
    }

    #[test]
    fn test_should_reject_empty_admins() {
        assert!(RolesManager::set_admins(vec![]).is_err());
    }

    #[test]
    fn test_should_reject_anonymous_admins() {
        assert!(RolesManager::set_admins(vec![Principal::anonymous()]).is_err());
    }

    #[test]
    fn test_should_override_admins() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(RolesManager::set_admins(vec![principal]).is_ok());
        assert_eq!(RolesManager::get_admins(), vec![principal,]);

        assert!(RolesManager::set_admins(vec![Principal::management_canister()]).is_ok());
        assert_eq!(
            RolesManager::get_admins(),
            vec![Principal::management_canister(),]
        );
    }

    #[test]
    fn test_should_tell_if_admin() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(RolesManager::set_admins(vec![principal]).is_ok());
        assert!(RolesManager::is_admin(principal));
        assert!(!RolesManager::is_admin(Principal::anonymous()));
    }
}
