mod configuration;
mod inspect;
mod liquidity_pool;
mod memory;
mod roles;
#[cfg(test)]
mod test_utils;

use candid::{Nat, Principal};
use did::ekoke::EkokeResult;
use did::ekoke_liquidity_pool::{
    EkokeLiquidityPoolInitData, LiquidityPoolAccounts, LiquidityPoolBalance,
};
use icrc::icrc1::account::Account;

use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::liquidity_pool::LiquidityPool;
use self::roles::RolesManager;
use crate::utils;

#[cfg(target_family = "wasm")]
pub const LIQUIDITY_POOL_SWAP_INTERVAL: std::time::Duration =
    std::time::Duration::from_secs(60 * 60 * 24); // 1 day

pub struct EkokeLiquidityPoolCanister;

impl EkokeLiquidityPoolCanister {
    pub fn init(args: EkokeLiquidityPoolInitData) {
        Configuration::set_ckbtc_canister(args.ckbtc_canister);
        Configuration::set_icp_ledger_canister(args.icp_ledger_canister);
        Configuration::set_swap_account(args.swap_account);
        Configuration::set_xrc_canister(args.xrc_canister);
        LiquidityPool::init();
        RolesManager::set_admins(args.admins).unwrap();

        Self::set_timers();
    }

    pub fn post_upgrade() {
        Self::set_timers();
    }

    /// Set application timers
    fn set_timers() {
        #[cfg(target_family = "wasm")]
        async fn swap_icp_to_btc_timer() {
            let xrc_principal = Configuration::get_xrc_canister();
            let _ = LiquidityPool::swap_icp_to_btc(xrc_principal).await;
        }
        // Liquidity pool ICP -> BTC swap timer
        #[cfg(target_family = "wasm")]
        ic_cdk_timers::set_timer_interval(LIQUIDITY_POOL_SWAP_INTERVAL, || {
            ic_cdk::spawn(swap_icp_to_btc_timer());
        });
    }

    /// Get liquidity pool balance from the different ledgers
    pub async fn liquidity_pool_balance() -> EkokeResult<LiquidityPoolBalance> {
        LiquidityPool::balance().await
    }

    /// Get liquidity pool accounts
    pub fn liquidity_pool_accounts() -> LiquidityPoolAccounts {
        LiquidityPool::accounts()
    }

    /// Returns cycles
    pub fn admin_cycles() -> Nat {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        utils::cycles()
    }

    /// Set swap account
    pub fn admin_set_swap_account(account: Account) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_swap_account(account);
    }

    /// Set ckbtc canister
    pub fn admin_set_ckbtc_canister(canister_id: Principal) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_ckbtc_canister(canister_id);
    }

    /// Set icp ledger canister
    pub fn admin_set_icp_ledger_canister(canister_id: Principal) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_icp_ledger_canister(canister_id);
    }

    pub fn admin_set_admins(admins: Vec<Principal>) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::set_admins(admins).unwrap();
    }

    /// Set xrc canister
    pub fn admin_set_xrc_canister(canister_id: Principal) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_xrc_canister(canister_id);
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use pretty_assertions::assert_eq;

    use super::test_utils::bob_account;
    use super::*;
    use crate::utils::caller;

    #[tokio::test]
    async fn test_should_init_canister() {
        init_canister();

        assert_eq!(RolesManager::get_admins(), vec![caller()]);

        // liquidity pool
        assert_eq!(LiquidityPool::accounts().ckbtc.owner, utils::id());
        assert!(LiquidityPool::accounts().ckbtc.subaccount.is_none());

        // swap account
        assert_eq!(Configuration::get_swap_account(), bob_account());

        // check canisters
        assert_eq!(Configuration::get_ckbtc_canister(), caller());
        assert_eq!(Configuration::get_icp_ledger_canister(), caller());
        assert_eq!(Configuration::get_xrc_canister(), caller());
    }

    #[tokio::test]
    async fn test_should_get_cycles() {
        init_canister();
        assert_eq!(EkokeLiquidityPoolCanister::admin_cycles(), utils::cycles());
    }

    #[test]
    fn test_should_set_xrc_canister() {
        init_canister();
        let canister_id = Principal::from_str("aaaaa-aa").unwrap();
        EkokeLiquidityPoolCanister::admin_set_xrc_canister(canister_id);
        assert_eq!(Configuration::get_xrc_canister(), canister_id);
    }

    #[test]
    fn test_should_set_ckbtc_canister() {
        init_canister();
        let canister_id = Principal::from_str("aaaaa-aa").unwrap();
        EkokeLiquidityPoolCanister::admin_set_ckbtc_canister(canister_id);
        assert_eq!(Configuration::get_ckbtc_canister(), canister_id);
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
            swap_account: bob_account(),
            ckbtc_canister: caller(),
            icp_ledger_canister: caller(),
            xrc_canister: caller(),
        };
        EkokeLiquidityPoolCanister::init(data);
    }
}
