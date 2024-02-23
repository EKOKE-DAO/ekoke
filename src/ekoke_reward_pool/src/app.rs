//! # App
//!
//! API implementation for deferred canister

mod configuration;
mod inspect;
mod ledger_client;
mod memory;
mod pool;
mod reward;
mod roles;
#[cfg(test)]
mod test_utils;

use candid::{Nat, Principal};
use did::ekoke::{AllowanceError, Ekoke, EkokeError, EkokeResult, PoolError};
use did::ekoke_reward_pool::{EkokeRewardPoolInitData, Role};
use did::ID;
use icrc::icrc1::account::Account;

use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::ledger_client::LedgerClient;
use self::pool::Pool;
use self::reward::Reward;
use self::roles::RolesManager;
use crate::constants::ICRC1_FEE;
use crate::utils;

pub struct EkokeRewardPoolCanister;

impl EkokeRewardPoolCanister {
    /// Init ekoke canister
    pub fn init(data: EkokeRewardPoolInitData) {
        // set canisters
        Configuration::set_ledger_canister(data.ledger_canister);
        // set roles
        if let Err(err) = RolesManager::set_admins(data.admins) {
            ic_cdk::trap(&format!("Error setting admins: {}", err));
        }
        // Set deferred canister
        RolesManager::give_role(data.deferred_canister, Role::DeferredCanister);
        // set marketplace canister
        RolesManager::give_role(data.marketplace_canister, Role::MarketplaceCanister);
    }

    /// Reserve a pool for the provided contract ID with the provided amount of $ekoke tokens.
    ///
    /// The tokens are withdrawned from the from's wallet.
    /// Obviously `from` wallet must be owned by the caller.
    pub async fn reserve_pool(
        contract_id: ID,
        amount: Ekoke,
        from_subaccount: Option<[u8; 32]>,
    ) -> EkokeResult<Ekoke> {
        let from_account = Account {
            owner: utils::caller(),
            subaccount: from_subaccount,
        };

        // check if given allowance is enough
        let min_allowance = amount.clone() + ICRC1_FEE;
        let given_allowance = LedgerClient::allowance(from_account).await?;
        if given_allowance < min_allowance {
            return Err(EkokeError::Allowance(AllowanceError::InsufficientFunds));
        }
        // try to send user tokens to the pool
        LedgerClient::transfer_from(from_account, amount.clone()).await?;

        Ok(Pool::reserve(&contract_id, amount))
    }

    /// Send reward to buyer reducing the balance from the pool associated to the contract, for the value of ekoke
    pub async fn send_reward(contract_id: ID, amount: Ekoke, buyer: Account) -> EkokeResult<()> {
        if !Inspect::inspect_is_marketplace_canister(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }

        if !Inspect::inspect_pool_exists(&contract_id) {
            return Err(EkokeError::Pool(PoolError::PoolNotFound(contract_id)));
        }

        Pool::withdraw_tokens(&contract_id, buyer, amount).await?;

        Ok(())
    }

    /// Get contract reward.
    ///
    /// This method can be called only by the deferred canister.
    ///
    /// If a pool is already reserved for the provided contract ID, the reserved amount will be returned.
    /// Otherwise, the provided amount will be reserved from canister wallet, if possible and returned.
    ///
    /// If the canister wallet doesn't have enough tokens to reserve `InsufficientBalance` error is returned
    pub async fn get_contract_reward(contract_id: ID, installments: u64) -> EkokeResult<Ekoke> {
        if !Inspect::inspect_is_deferred_canister(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }

        Reward::get_contract_reward(contract_id, installments).await
    }

    // # admin methods

    /// Set role to the provided principal
    pub fn admin_set_role(principal: Principal, role: Role) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::give_role(principal, role)
    }

    /// Remove role from the provided principal
    pub fn admin_remove_role(principal: Principal, role: Role) -> EkokeResult<()> {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::remove_role(principal, role)
    }

    /// Returns cycles
    pub fn admin_cycles() -> Nat {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        utils::cycles()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::{assert_eq, assert_ne};

    use super::test_utils::bob_account;
    use super::*;
    use crate::constants::MIN_REWARD;
    use crate::utils::caller;

    #[tokio::test]
    async fn test_should_init_canister() {
        init_canister();

        assert_ne!(Configuration::get_ledger_canister(), Principal::anonymous());
        assert_eq!(RolesManager::get_admins(), vec![caller()]);
        assert!(RolesManager::has_role(caller(), Role::DeferredCanister));

        assert_eq!(Configuration::get_ledger_canister(), caller());
    }

    #[tokio::test]
    async fn test_should_reserve_pool() {
        init_canister();
        let contract_id = 1_u64.into();
        let amount_amount: Nat = 1000_u64.into();

        let result = EkokeRewardPoolCanister::reserve_pool(
            contract_id,
            amount_amount.clone(),
            test_utils::caller_account().subaccount,
        )
        .await;

        assert_eq!(result, Ok(amount_amount));
    }

    #[tokio::test]
    async fn test_should_not_allow_reserve_pool() {
        init_canister();
        let contract_id = 1_u64.into();
        let amount_amount = 1000_u64.into();

        assert!(
            EkokeRewardPoolCanister::reserve_pool(contract_id, amount_amount, Some([3; 32]),)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_should_send_reward() {
        init_canister();
        let contract_id: ID = 1_u64.into();

        let amount_amount: Nat = 500_000_u64.into();

        let result = EkokeRewardPoolCanister::reserve_pool(
            contract_id.clone(),
            amount_amount.clone(),
            test_utils::caller_account().subaccount,
        )
        .await;

        assert_eq!(result, Ok(amount_amount));

        // send reward to bob
        assert!(EkokeRewardPoolCanister::send_reward(
            contract_id,
            MIN_REWARD.into(),
            bob_account()
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn test_should_not_send_reward() {
        init_canister();
        let contract_id: ID = 1_u64.into();

        let amount_amount: Nat = 20_000_u64.into();

        let result = EkokeRewardPoolCanister::reserve_pool(
            contract_id.clone(),
            amount_amount.clone(),
            test_utils::caller_account().subaccount,
        )
        .await;

        assert_eq!(result, Ok(amount_amount));

        // send reward to bob
        assert!(EkokeRewardPoolCanister::send_reward(
            contract_id,
            50_000_u64.into(),
            bob_account()
        )
        .await
        .is_err());
        assert!(
            EkokeRewardPoolCanister::send_reward(2_u64.into(), 500_u64.into(), bob_account())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_should_set_role() {
        init_canister();
        let principal = Principal::management_canister();
        let role = Role::Admin;
        EkokeRewardPoolCanister::admin_set_role(principal, role);
        assert!(RolesManager::is_admin(principal));
    }

    #[tokio::test]
    async fn test_should_remove_role() {
        init_canister();
        let principal = Principal::management_canister();
        let role = Role::Admin;
        EkokeRewardPoolCanister::admin_set_role(principal, role);
        assert!(RolesManager::is_admin(principal));
        EkokeRewardPoolCanister::admin_remove_role(principal, role).unwrap();
        assert!(!RolesManager::is_admin(principal));
    }

    #[tokio::test]
    async fn test_should_get_cycles() {
        init_canister();
        assert_eq!(EkokeRewardPoolCanister::admin_cycles(), utils::cycles());
    }

    fn init_canister() {
        let data = EkokeRewardPoolInitData {
            admins: vec![caller()],
            deferred_canister: caller(),
            marketplace_canister: caller(),
            ledger_canister: caller(),
        };
        EkokeRewardPoolCanister::init(data);
    }
}
