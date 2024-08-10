mod configuration;
mod inspect;
mod liquidity_pool;
mod memory;
mod refund;
mod roles;

use std::collections::HashMap;

use candid::{Nat, Principal};
use did::ekoke::EkokeResult;
use did::ekoke_liquidity_pool::{
    EkokeLiquidityPoolInitData, LiquidityPoolAccounts, LiquidityPoolBalance, WithdrawError,
};
use icrc::icrc1::account::{Account, Subaccount};

use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::liquidity_pool::LiquidityPool;
use self::refund::Refund;
use self::roles::RolesManager;
use crate::utils;

pub struct EkokeLiquidityPoolCanister;

impl EkokeLiquidityPoolCanister {
    pub fn init(args: EkokeLiquidityPoolInitData) {
        Configuration::set_icp_ledger_canister(args.icp_ledger_canister);
        Configuration::set_deferred_canister(args.deferred_canister_id);
        LiquidityPool::init();
        RolesManager::set_admins(args.admins).unwrap();
    }

    /// Get liquidity pool balance from the different ledgers
    pub async fn liquidity_pool_balance() -> EkokeResult<LiquidityPoolBalance> {
        LiquidityPool::balance().await
    }

    /// Get liquidity pool accounts
    pub fn liquidity_pool_accounts() -> LiquidityPoolAccounts {
        LiquidityPool::accounts()
    }

    /// Create refunds
    pub async fn create_refunds(refunds: HashMap<Principal, Nat>) {
        if !Inspect::inspect_is_deferred_canister(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }

        for (principal, amount) in refunds {
            Refund::insert_or_update(principal, amount);
        }
    }

    /// Withdraw refund
    pub async fn withdraw_refund(subaccount: Option<Subaccount>) -> Result<(), WithdrawError> {
        let caller = utils::caller();
        // get refund amount
        let Some(refund_amount) = Refund::get(caller) else {
            return Err(WithdrawError::NothingToWithdraw(caller));
        };
        let account = Account {
            owner: caller,
            subaccount,
        };

        // withdraw icp
        LiquidityPool::withdraw_icp(account, refund_amount).await?;

        // remove refund
        Refund::remove(caller);

        Ok(())
    }

    /// Returns cycles
    pub fn admin_cycles() -> Nat {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        utils::cycles()
    }

    /// Set icp ledger canister
    pub fn admin_set_icp_ledger_canister(canister_id: Principal) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_icp_ledger_canister(canister_id);
    }

    pub fn admin_set_deferred_canister(canister_id: Principal) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_deferred_canister(canister_id);
    }

    pub fn admin_set_admins(admins: Vec<Principal>) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::set_admins(admins).unwrap();
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::utils::caller;

    #[tokio::test]
    async fn test_should_init_canister() {
        init_canister();

        assert_eq!(RolesManager::get_admins(), vec![caller()]);

        // liquidity pool
        assert_eq!(LiquidityPool::accounts().icp.owner, utils::id());
        assert!(LiquidityPool::accounts().icp.subaccount.is_none());

        // check canisters
        assert_eq!(Configuration::get_icp_ledger_canister(), caller());
    }

    #[tokio::test]
    async fn test_should_withdraw_icp() {
        init_canister();

        // insert refund
        Refund::insert_or_update(caller(), Nat::from(100u64));

        // withdraw refund
        EkokeLiquidityPoolCanister::withdraw_refund(None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_should_get_cycles() {
        init_canister();
        assert_eq!(EkokeLiquidityPoolCanister::admin_cycles(), utils::cycles());
    }

    #[test]
    fn test_should_set_icp_ledger_canister() {
        init_canister();
        let canister_id = Principal::from_str("aaaaa-aa").unwrap();
        EkokeLiquidityPoolCanister::admin_set_icp_ledger_canister(canister_id);
        assert_eq!(Configuration::get_icp_ledger_canister(), canister_id);
    }

    #[test]
    fn test_should_set_admins() {
        init_canister();
        let admins = vec![Principal::from_str("aaaaa-aa").unwrap()];
        EkokeLiquidityPoolCanister::admin_set_admins(admins.clone());
        assert_eq!(RolesManager::get_admins(), admins);
    }

    fn init_canister() {
        let data = EkokeLiquidityPoolInitData {
            admins: vec![caller()],
            icp_ledger_canister: caller(),
            deferred_canister_id: caller(),
        };
        EkokeLiquidityPoolCanister::init(data);
    }
}
