//! The liquidity pool provides the access to the internal "deposit-only" pool of liquidity that
//! backs the value of the Fly token.
//! The pool can both contain ICP and ckBTC.
// ! The pool is not owned by anyone, and is not controlled by anyone, except the canister.

mod ckbtc;
mod icp_ledger;

use std::cell::RefCell;

use candid::Principal;
use did::fly::{FlyResult, LiquidityPoolAccounts, LiquidityPoolBalance};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use icrc::icrc1::account::Account;

use self::ckbtc::CkBtc;
use self::icp_ledger::IcpLedger;
use super::balance::StorableAccount;
use crate::app::memory::{
    LIQUIDITY_POOL_ACCOUNT_MEMORY_ID, LIQUIDITY_POOL_CKBTC_ACCOUNT_MEMORY_ID, MEMORY_MANAGER,
};
use crate::utils::{self, random_subaccount};

thread_local! {
    /// ICP ledger account
    static ICP_ACCOUNT: RefCell<StableCell<Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(LIQUIDITY_POOL_ACCOUNT_MEMORY_ID)),
            vec![]).unwrap()
    );

    /// Pool map is an association between a contract-id and the account which holds the pool for that contract.
    /// There is an account for each contract.
    static CKBTC_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(LIQUIDITY_POOL_CKBTC_ACCOUNT_MEMORY_ID)),
            Account {
                owner: Principal::anonymous(),
                subaccount: None,
            }.into()).unwrap()
    );
}

pub struct LiquidityPool;

impl LiquidityPool {
    /// Init liquidity pool
    pub async fn init() {
        // generate CkBTC account
        CKBTC_ACCOUNT.with_borrow_mut(|account| {
            account
                .set(
                    Account {
                        owner: utils::id(),
                        subaccount: Some(random_subaccount()),
                    }
                    .into(),
                )
                .unwrap();
        });
        // get account from ICP ledger
        let icp_account = IcpLedger::account_identifier(utils::id()).await;
        ICP_ACCOUNT.with_borrow_mut(|account| {
            account.set(icp_account).unwrap();
        });
    }

    /// Get liquidity pool accounts
    pub fn accounts() -> LiquidityPoolAccounts {
        LiquidityPoolAccounts {
            icp: ICP_ACCOUNT.with_borrow(|account| account.get().clone()),
            ckbtc: CKBTC_ACCOUNT.with_borrow(|account| account.get().clone()).0,
        }
    }

    /// Get liquidity pool balance
    pub async fn balance() -> FlyResult<LiquidityPoolBalance> {
        let accounts = Self::accounts();
        let icp = IcpLedger::account_balance(accounts.icp).await?;
        let ckbtc = CkBtc::icrc1_balance_of(accounts.ckbtc).await?;

        Ok(LiquidityPoolBalance { icp, ckbtc })
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn test_should_init_and_get_accounts() {
        LiquidityPool::init().await;
        let account = LiquidityPool::accounts();
        assert_eq!(
            account.ckbtc,
            CKBTC_ACCOUNT.with_borrow(|account| account.get().clone()).0
        );
        assert_eq!(
            account.icp,
            ICP_ACCOUNT.with_borrow(|account| account.get().clone())
        );
    }

    #[tokio::test]
    async fn test_should_get_balance() {
        LiquidityPool::init().await;
        let balance = LiquidityPool::balance().await.unwrap();
        assert_eq!(balance.ckbtc, 88_378_u64);
        assert_eq!(balance.icp, 1_216_794_022);
    }
}
