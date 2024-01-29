//! Roles

use std::cell::RefCell;

use candid::Principal;
use did::ekoke::{ConfigurationError, EkokeError, EkokeResult, Role, Roles};
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

    /// Returns whether principal has provided role
    pub fn has_role(principal: Principal, role: Role) -> bool {
        Self::with_principal(principal, |roles| roles.0.contains(&role)).unwrap_or(false)
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

    /// Give a certain principal the provided role
    pub fn give_role(principal: Principal, role: Role) {
        Self::with_principal_mut(principal, |roles| {
            if !roles.0.contains(&role) {
                roles.0.push(role);
            }
        });
    }

    /// Remove a role from the provided role.
    /// Fails if trying to remove the only admin of the canister
    pub fn remove_role(principal: Principal, role: Role) -> EkokeResult<()> {
        let admins = Self::get_admins();
        if admins.len() == 1 && admins.contains(&principal) && role == Role::Admin {
            return Err(EkokeError::Configuration(
                ConfigurationError::AdminsCantBeEmpty,
            ));
        }

        Self::with_principal_mut(principal, |roles| {
            roles.0.retain(|r| r != &role);
        });

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

    #[test]
    fn test_should_tell_if_has_role() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        assert!(!RolesManager::has_role(principal, Role::DeferredCanister));
        RolesManager::give_role(principal, Role::DeferredCanister);
        assert!(RolesManager::has_role(principal, Role::DeferredCanister));
    }

    #[test]
    fn test_should_give_role() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(!RolesManager::is_admin(principal));
        RolesManager::give_role(principal, Role::Admin);
        assert!(RolesManager::is_admin(principal));
    }

    #[test]
    fn test_should_remove_role() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(!RolesManager::is_admin(principal));
        RolesManager::give_role(principal, Role::Admin);
        assert!(RolesManager::is_admin(principal));

        RolesManager::give_role(Principal::management_canister(), Role::Admin);
        assert!(RolesManager::remove_role(principal, Role::Admin).is_ok());
        assert!(!RolesManager::is_admin(principal));
    }

    #[test]
    fn test_should_not_allow_to_remove_the_only_admin() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(RolesManager::set_admins(vec![principal]).is_ok());
        assert!(RolesManager::is_admin(principal));

        assert!(RolesManager::remove_role(principal, Role::Admin).is_err());
        assert!(RolesManager::is_admin(principal));
    }
}
