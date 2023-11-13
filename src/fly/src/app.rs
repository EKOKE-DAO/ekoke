//! # App
//!
//! API implementation for dilazionato canister

mod balances;
mod configuration;
mod inspect;
mod memory;
mod pool;
mod roles;

use candid::Principal;
use did::fly::{FlyInitData, FlyResult, Role};
use did::ID;

use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::pool::Pool;
use self::roles::RolesManager;
use crate::utils::caller;

pub struct FlyCanister;

impl FlyCanister {
    /// Init fly canister
    pub fn init(data: FlyInitData) {
        Configuration::set_minting_account(data.minting_account);
        if let Err(err) = RolesManager::set_admins(data.admins) {
            ic_cdk::trap(&format!("Error setting admins: {}", err));
        }
    }

    pub fn post_upgrade() {}

    /// Reserve a pool for the provided contract ID with the provided amount of $picoFly tokens
    pub fn reserve_pool(contract_id: ID, picofly_amount: u64) -> FlyResult<u64> {
        Pool::reserve(&contract_id, picofly_amount)
    }

    /// Set role to the provided principal
    pub fn admin_set_role(principal: Principal, role: Role) {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::give_role(principal, role)
    }

    /// Remove role from the provided principal
    pub fn admin_remove_role(principal: Principal, role: Role) -> FlyResult<()> {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::remove_role(principal, role)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::utils::caller;

    #[test]
    fn test_should_reserve_pool() {
        init_canister();
        let contract_id = 1.into();
        let picofly_amount = 1000;

        let result = FlyCanister::reserve_pool(contract_id, picofly_amount);

        assert_eq!(result, Ok(picofly_amount));
    }

    #[test]
    fn test_should_set_role() {
        init_canister();
        let principal = Principal::management_canister();
        let role = Role::Admin;
        FlyCanister::admin_set_role(principal, role);
        assert!(RolesManager::is_admin(principal));
    }

    #[test]
    fn test_should_remove_role() {
        init_canister();
        let principal = Principal::management_canister();
        let role = Role::Admin;
        FlyCanister::admin_set_role(principal, role);
        assert!(RolesManager::is_admin(principal));
        FlyCanister::admin_remove_role(principal, role).unwrap();
        assert!(!RolesManager::is_admin(principal));
    }

    fn init_canister() {
        let data = FlyInitData {
            admins: vec![caller()],
            minting_account: caller(),
        };
        FlyCanister::init(data);
    }
}
