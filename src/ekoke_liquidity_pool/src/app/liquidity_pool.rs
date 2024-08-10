//! The liquidity pool provides the access to the internal "deposit-only" pool of liquidity that
//! backs the value of the Ekoke token.

use std::cell::RefCell;

use candid::{Nat, Principal};
use did::ekoke::{EkokeError, EkokeResult};
use did::ekoke_liquidity_pool::{LiquidityPoolAccounts, LiquidityPoolBalance, WithdrawError};
use did::StorableAccount;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use icrc::icrc1::account::Account;
use icrc::icrc1::transfer::TransferError;
use icrc::IcrcLedgerClient;

use crate::app::configuration::Configuration;
use crate::app::memory::{LIQUIDITY_POOL_ACCOUNT_MEMORY_ID, MEMORY_MANAGER};
use crate::utils;

thread_local! {
    /// ICP ledger account
    static ICP_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(LIQUIDITY_POOL_ACCOUNT_MEMORY_ID)),
            Account {
                owner: Principal::anonymous(),
                subaccount: None,
            }.into()).unwrap()
    );
}

pub struct LiquidityPool;

impl LiquidityPool {
    /// Init liquidity pool
    pub fn init() {
        // generate ICP account
        ICP_ACCOUNT.with_borrow_mut(|account| {
            account
                .set(
                    Account {
                        owner: utils::id(),
                        subaccount: None,
                    }
                    .into(),
                )
                .unwrap();
        });
    }

    /// Get liquidity pool accounts
    pub fn accounts() -> LiquidityPoolAccounts {
        LiquidityPoolAccounts {
            icp: ICP_ACCOUNT.with_borrow(|account| *account.get()).0,
        }
    }

    /// Get liquidity pool balance
    pub async fn balance() -> EkokeResult<LiquidityPoolBalance> {
        let accounts = Self::accounts();
        let icp_client = IcrcLedgerClient::from(Configuration::get_icp_ledger_canister());

        let icp_balance = icp_client
            .icrc1_balance_of(accounts.icp)
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?;

        Ok(LiquidityPoolBalance { icp: icp_balance })
    }

    /// Withdraw ICP from the liquidity pool
    pub async fn withdraw_icp(account: Account, amount: Nat) -> Result<(), WithdrawError> {
        let icp_ledger_client = IcrcLedgerClient::from(Configuration::get_icp_ledger_canister());

        let icp_fee = icp_ledger_client
            .icrc1_fee()
            .await
            .map_err(|(code, msg)| WithdrawError::CanisterCall(code, msg))?;

        // verify the balance
        let balance = Self::balance().await.expect("failed to get balance").icp;
        let required_balance = amount.clone() + icp_fee;

        // check if the balance is sufficient
        if balance < required_balance {
            return Err(TransferError::InsufficientFunds { balance }.into());
        }

        // transfer
        icp_ledger_client
            .icrc1_transfer(account, amount, None)
            .await
            .map_err(|(code, msg)| WithdrawError::CanisterCall(code, msg))??;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_init_and_get_accounts() {
        LiquidityPool::init();
        let account = LiquidityPool::accounts();
        assert_eq!(account.icp.owner, utils::id());

        assert_eq!(
            account.icp,
            ICP_ACCOUNT.with_borrow(|account| account.get().clone()).0
        );
    }

    #[tokio::test]
    async fn test_should_get_balance() {
        LiquidityPool::init();
        let balance = LiquidityPool::balance().await.unwrap();
        assert_eq!(balance.icp, 888010101000000u64);
    }

    #[tokio::test]
    async fn test_should_withdraw_icp() {
        LiquidityPool::init();
        let account = Account {
            owner: utils::id(),
            subaccount: None,
        };
        let amount = Nat::from(100u64);

        LiquidityPool::withdraw_icp(account, amount).await.unwrap();
    }
}
