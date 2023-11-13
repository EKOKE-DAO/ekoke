//! # App
//!
//! API implementation for dilazionato canister

mod balance;
mod configuration;
mod inspect;
mod memory;
mod pool;
mod reward;
mod roles;
#[cfg(test)]
mod test_utils;

use candid::Principal;
use did::fly::{FlyInitData, FlyResult, PicoFly, Role};
use did::ID;
use icrc::icrc1::account::Account;

use self::balance::Balance;
use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::pool::Pool;
use self::reward::Reward;
use self::roles::RolesManager;
use crate::utils;

pub struct FlyCanister;

impl FlyCanister {
    /// Init fly canister
    pub fn init(data: FlyInitData) {
        Configuration::set_minting_account(data.minting_account);
        if let Err(err) = RolesManager::set_admins(data.admins) {
            ic_cdk::trap(&format!("Error setting admins: {}", err));
        }
        // Set dilazionato canister
        RolesManager::give_role(data.dilazionato_canister, Role::DilazionatoCanister);
        // init balances
        Balance::init_balances(
            utils::fly_to_picofly(data.total_supply),
            data.initial_balances,
        );
    }

    pub fn post_upgrade() {}

    /// Reserve a pool for the provided contract ID with the provided amount of $picoFly tokens.
    ///
    /// The tokens are withdrawned from the from's wallet.
    /// Obviously `from` wallet must be owned by the caller.
    pub fn reserve_pool(from: Account, contract_id: ID, picofly_amount: u64) -> FlyResult<u64> {
        if !Inspect::inspect_caller_owns_wallet(utils::caller(), from) {
            ic_cdk::trap("You don't own this account");
        }

        Pool::reserve(&contract_id, from, picofly_amount)
    }

    /// Get contract reward.
    ///
    /// This method can be called only by the dilazionato canister.
    ///
    /// If a pool is already reserved for the provided contract ID, the reserved amount will be returned.
    /// Otherwise, the provided amount will be reserved from canister wallet, if possible and returned.
    ///
    /// If the canister wallet doesn't have enough tokens to reserve `InsufficientBalance` error is returned
    pub fn get_contract_reward(contract_id: ID, installments: u64) -> FlyResult<PicoFly> {
        if !Inspect::inspect_is_dilazionato_canister(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }

        Reward::get_contract_reward(contract_id, installments)
    }

    /// Set role to the provided principal
    pub fn admin_set_role(principal: Principal, role: Role) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::give_role(principal, role)
    }

    /// Remove role from the provided principal
    pub fn admin_remove_role(principal: Principal, role: Role) -> FlyResult<()> {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::remove_role(principal, role)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::test_utils::{alice_account, bob_account, caller_account};
    use super::*;
    use crate::utils::{caller, fly_to_picofly};

    #[test]
    fn test_should_init_canister() {
        init_canister();

        assert_eq!(Configuration::get_minting_account(), caller());
        assert_eq!(RolesManager::get_admins(), vec![caller()]);
        assert!(RolesManager::has_role(caller(), Role::DilazionatoCanister));
        // init balance
        assert_eq!(
            Balance::balance_of(alice_account()).unwrap(),
            fly_to_picofly(50_000)
        );
        assert_eq!(
            Balance::balance_of(bob_account()).unwrap(),
            fly_to_picofly(50_000)
        );
        assert_eq!(
            Balance::balance_of(caller_account()).unwrap(),
            fly_to_picofly(100_000)
        );
        // supply
        assert_eq!(
            Balance::balance_of(Balance::canister_wallet_account()).unwrap(),
            fly_to_picofly(8_688_888)
        );
    }

    #[test]
    fn test_should_reserve_pool() {
        init_canister();
        let contract_id = 1.into();
        let picofly_amount = 1000;

        let result =
            FlyCanister::reserve_pool(test_utils::caller_account(), contract_id, picofly_amount);

        assert_eq!(result, Ok(picofly_amount));
    }

    #[test]
    #[should_panic]
    fn test_should_not_allow_reserve_pool() {
        init_canister();
        let contract_id = 1.into();
        let picofly_amount = 1000;

        assert!(
            FlyCanister::reserve_pool(test_utils::bob_account(), contract_id, picofly_amount)
                .is_err()
        );
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
            total_supply: 8_888_888,
            dilazionato_canister: caller(),
            initial_balances: vec![
                (alice_account(), fly_to_picofly(50_000)),
                (bob_account(), fly_to_picofly(50_000)),
                (caller_account(), fly_to_picofly(100_000)),
            ],
        };
        FlyCanister::init(data);
    }
}
