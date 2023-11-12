//! Roles

use std::cell::RefCell;

use candid::Principal;
use did::dilazionato::{ConfigurationError, DilazionatoError, DilazionatoResult, Role, Roles};
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
    /// Returns whether principal is custodian
    pub fn is_custodian(principal: Principal) -> bool {
        Self::with_principal(principal, |roles| roles.0.contains(&Role::Custodian)).unwrap_or(false)
    }

    /// Returns whether principal is agent
    pub fn is_agent(principal: Principal) -> bool {
        Self::with_principal(principal, |roles| roles.0.contains(&Role::Agent)).unwrap_or(false)
    }

    /// Get canister custodians
    pub fn get_custodians() -> Vec<Principal> {
        CANISTER_ROLES.with_borrow(|roles_map| {
            roles_map
                .iter()
                .filter(|(_, roles)| roles.0.contains(&Role::Custodian))
                .map(|(principal, _)| principal.0)
                .collect()
        })
    }

    /// Set canister custodians.
    ///
    /// WARNING: previous custodians will be overwritten
    pub fn set_custodians(custodians: Vec<Principal>) -> DilazionatoResult<()> {
        // check if custodians is empty
        if custodians.is_empty() {
            return Err(DilazionatoError::Configuration(
                ConfigurationError::CustodialsCantBeEmpty,
            ));
        }

        // check if principal is anonymous
        if custodians
            .iter()
            .any(|principal| principal == &Principal::anonymous())
        {
            return Err(DilazionatoError::Configuration(
                ConfigurationError::AnonymousCustodial,
            ));
        }

        // remove current custodians
        let current_custodians = Self::get_custodians();
        CANISTER_ROLES.with_borrow_mut(|roles| {
            for principal in current_custodians {
                roles.remove(&principal.into());
            }
        });

        for principal in custodians {
            Self::with_principal_mut(principal, |roles| {
                if !roles.0.contains(&Role::Custodian) {
                    roles.0.push(Role::Custodian);
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
    /// Fails if trying to remove the only custodian of the canister
    pub fn remove_role(principal: Principal, role: Role) -> DilazionatoResult<()> {
        let custodians = Self::get_custodians();
        if custodians.len() == 1 && custodians.contains(&principal) && role == Role::Custodian {
            return Err(DilazionatoError::Configuration(
                ConfigurationError::CustodialsCantBeEmpty,
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
    fn test_should_set_and_get_canister_custodians() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        assert!(RolesManager::get_custodians().is_empty());
        assert!(RolesManager::set_custodians(vec![principal]).is_ok());
        assert_eq!(RolesManager::get_custodians(), vec![principal,]);
    }

    #[test]
    fn test_should_reject_empty_custodians() {
        assert!(RolesManager::set_custodians(vec![]).is_err());
    }

    #[test]
    fn test_should_reject_anonymous_custodians() {
        assert!(RolesManager::set_custodians(vec![Principal::anonymous()]).is_err());
    }

    #[test]
    fn test_should_override_custodians() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(RolesManager::set_custodians(vec![principal]).is_ok());
        assert_eq!(RolesManager::get_custodians(), vec![principal,]);

        assert!(RolesManager::set_custodians(vec![Principal::management_canister()]).is_ok());
        assert_eq!(
            RolesManager::get_custodians(),
            vec![Principal::management_canister(),]
        );
    }

    #[test]
    fn test_should_tell_if_custodian() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(RolesManager::set_custodians(vec![principal]).is_ok());
        assert!(RolesManager::is_custodian(principal));
        assert!(!RolesManager::is_custodian(Principal::anonymous()));
    }

    #[test]
    fn test_should_give_role() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(!RolesManager::is_agent(principal));
        RolesManager::give_role(principal, Role::Agent);
        assert!(RolesManager::is_agent(principal));
    }

    #[test]
    fn test_should_remove_role() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(!RolesManager::is_agent(principal));
        RolesManager::give_role(principal, Role::Agent);
        assert!(RolesManager::is_agent(principal));

        assert!(RolesManager::remove_role(principal, Role::Agent).is_ok());
        assert!(!RolesManager::is_agent(principal));
    }

    #[test]
    fn test_should_not_allow_to_remove_the_only_custodian() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert!(RolesManager::set_custodians(vec![principal]).is_ok());
        assert!(RolesManager::is_custodian(principal));

        assert!(RolesManager::remove_role(principal, Role::Custodian).is_err());
        assert!(RolesManager::is_custodian(principal));
    }
}
