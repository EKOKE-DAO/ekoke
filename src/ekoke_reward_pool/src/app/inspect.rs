//! # Inspect
//!
//! Deferred inspect message handler

use candid::Principal;
use did::ekoke_reward_pool::Role;
use did::ID;

use super::pool::Pool;
use super::roles::RolesManager;

pub struct Inspect;

impl Inspect {
    /// Returns whether caller is custodian of the canister
    pub fn inspect_is_admin(caller: Principal) -> bool {
        RolesManager::is_admin(caller)
    }

    /// Returns whether caller is deferred canister
    pub fn inspect_is_deferred_canister(caller: Principal) -> bool {
        RolesManager::has_role(caller, Role::DeferredCanister)
    }

    /// Returns whether caller is marketplace canister
    pub fn inspect_is_marketplace_canister(caller: Principal) -> bool {
        RolesManager::has_role(caller, Role::MarketplaceCanister)
    }

    /// inspect whether pool exists
    pub fn inspect_pool_exists(contract_id: &ID) -> bool {
        Pool::has_pool(contract_id)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_inspect_is_custodian() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_admin(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert_eq!(Inspect::inspect_is_admin(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert!(RolesManager::set_admins(vec![caller]).is_ok());
        assert_eq!(Inspect::inspect_is_admin(caller), true);
    }

    #[test]
    fn test_should_inspect_is_deferred() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_deferred_canister(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        RolesManager::give_role(caller, Role::DeferredCanister);
        assert_eq!(Inspect::inspect_is_deferred_canister(caller), true);
    }

    #[test]
    fn test_should_inspect_pool_exists() {
        let contract_id = ID::from(0_u64);
        assert!(!Inspect::inspect_pool_exists(&contract_id));

        Pool::reserve(&contract_id, 100_u64.into());

        assert!(Inspect::inspect_pool_exists(&contract_id));
    }
}
