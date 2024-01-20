//! Roles

use std::cell::RefCell;

use candid::Principal;
use did::marketplace::{ConfigurationError, MarketplaceError, MarketplaceResult, Role, Roles};
use did::StorablePrincipal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};

use crate::app::memory::{MEMORY_MANAGER, ROLES_MEMORY_ID};

thread_local! {
    /// Principals that can manage the canister
    static CANISTER_ROLES: RefCell<StableBTreeMap<StorablePrincipal, Roles, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(ROLES_MEMORY_ID)))
    );
}
pub struct RolesManager;

impl RolesManager {
    /// Returns whether principal is admin
    pub fn is_admin(principal: Principal) -> bool {
        Self::with_principal(principal, |roles| roles.0.contains(&Role::Admin)).unwrap_or(false)
    }

    /// Get canister admins
    pub fn get_admins() -> Vec<Principal> {
        CANISTER_ROLES.with_borrow(|roles_map| {
            roles_map
                .iter()
                .filter(|(_, roles)| roles.0.contains(&Role::Admin))
                .map(|(principal, _)| principal.0)
                .collect()
        })
    }

    /// Set canister admins.
    ///
    /// WARNING: previous admins will be overwritten
    pub fn set_admins(admins: Vec<Principal>) -> MarketplaceResult<()> {
        // check if admins is empty
        if admins.is_empty() {
            return Err(MarketplaceError::Configuration(
                ConfigurationError::AdminsCantBeEmpty,
            ));
        }

        // check if principal is anonymous
        if admins
            .iter()
            .any(|principal| principal == &Principal::anonymous())
        {
            return Err(MarketplaceError::Configuration(
                ConfigurationError::AnonymousAdmin,
            ));
        }

        // remove current admins
        let current_admins = Self::get_admins();
        CANISTER_ROLES.with_borrow_mut(|roles| {
            for principal in current_admins {
                roles.remove(&principal.into());
            }
        });

        for principal in admins {
            Self::with_principal_mut(principal, |roles| {
                if !roles.0.contains(&Role::Admin) {
                    roles.0.push(Role::Admin);
                }
            });
        }

        Ok(())
    }

    fn with_principal<F, T>(principal: Principal, f: F) -> Option<T>
    where
        F: FnOnce(&Roles) -> T,
    {
        CANISTER_ROLES
            .with_borrow(|roles_map| roles_map.get(&principal.into()).map(|roles| f(&roles)))
    }

    fn with_principal_mut<F, T>(principal: Principal, f: F) -> T
    where
        F: FnOnce(&mut Roles) -> T,
    {
        CANISTER_ROLES.with_borrow_mut(|roles_map| {
            let mut roles = match roles_map.get(&principal.into()) {
                Some(roles) => roles.clone(),
                None => vec![].into(),
            };
            let res = f(&mut roles);
            roles_map.insert(principal.into(), roles);

            res
        })
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
