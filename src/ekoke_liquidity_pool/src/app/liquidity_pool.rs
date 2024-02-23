//! The liquidity pool provides the access to the internal "deposit-only" pool of liquidity that
//! backs the value of the Ekoke token.
//! The pool can both contain ICP and ckBTC.
// ! The pool is not owned by anyone, and is not controlled by anyone, except the canister.

mod xrc;

use std::cell::RefCell;

use candid::{Nat, Principal};
use did::ekoke::{BalanceError, EkokeError, EkokeResult};
use did::ekoke_liquidity_pool::{LiquidityPoolAccounts, LiquidityPoolBalance};
use did::StorableAccount;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use icrc::icrc1::account::Account;
use icrc::IcrcLedgerClient;

use self::xrc::Xrc;
use crate::app::configuration::Configuration;
use crate::app::memory::{
    LIQUIDITY_POOL_ACCOUNT_MEMORY_ID, LIQUIDITY_POOL_CKBTC_ACCOUNT_MEMORY_ID, MEMORY_MANAGER,
};
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
    pub fn init() {
        // generate CkBTC account
        CKBTC_ACCOUNT.with_borrow_mut(|account| {
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
        // generate CkBTC account
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
            ckbtc: CKBTC_ACCOUNT.with_borrow(|account| *account.get()).0,
        }
    }

    /// Get liquidity pool balance
    pub async fn balance() -> EkokeResult<LiquidityPoolBalance> {
        let accounts = Self::accounts();
        let icp_client = IcrcLedgerClient::from(Configuration::get_icp_ledger_canister());
        let ckbtc_client = IcrcLedgerClient::from(Configuration::get_ckbtc_canister());

        let icp_balance = icp_client
            .icrc1_balance_of(accounts.icp)
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?;
        let ckbtc_balance = ckbtc_client
            .icrc1_balance_of(accounts.ckbtc)
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?;

        Ok(LiquidityPoolBalance {
            icp: icp_balance,
            ckbtc: ckbtc_balance,
        })
    }

    /// Swap the current liquidity pool in ICP to BTC using the swap account
    #[allow(dead_code)]
    pub async fn swap_icp_to_btc(xrc_principal: Principal) -> EkokeResult<()> {
        // get the current exchange rate ICP/BTC
        let rate = Xrc::get_icp_to_btc_rate(xrc_principal).await?;
        let ckbtc_client = IcrcLedgerClient::from(Configuration::get_ckbtc_canister());
        // get current balance of swap account of CKBTC
        let swap_account_balance = ckbtc_client
            .icrc1_balance_of(Configuration::get_swap_account())
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?;
        // get current ICP balance of the liquidity pool
        let accounts = Self::accounts();
        let liquidity_pool_balance = Self::balance().await?;

        // check ckbtc allowance
        let allowance = ckbtc_client
            .icrc2_allowance(accounts.ckbtc, Configuration::get_swap_account())
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?
            .allowance;

        // get amounts to trade
        let amounts = Self::get_exchange_amounts(
            rate,
            allowance,
            swap_account_balance,
            liquidity_pool_balance.icp.clone(),
        );

        // check ICP balance
        if liquidity_pool_balance.icp < amounts.icp {
            // abort
            return Err(EkokeError::Balance(BalanceError::InsufficientBalance));
        }

        // send ICP to swap account
        let icp_client = IcrcLedgerClient::from(Configuration::get_icp_ledger_canister());
        icp_client
            .icrc1_transfer(
                Configuration::get_swap_account(),
                liquidity_pool_balance.icp,
            )
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?
            .map_err(EkokeError::Icrc1Transfer)?;
        // send BTC to liquidity pool
        ckbtc_client
            .icrc2_transfer_from(
                accounts.ckbtc.subaccount,
                Configuration::get_swap_account(),
                accounts.ckbtc,
                amounts.btc,
            )
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?
            .map_err(EkokeError::Icrc2Transfer)?;

        Ok(())
    }

    /// given exchange rates, the maximum allowance and the swap account balance and the liquidity pool balance
    /// calculate the values to exchange in both ICP and BTC.
    /// The values must match if converted to the same currency.
    fn get_exchange_amounts(
        icp_btc_rate: f64,
        allowance: Nat,
        swap_account_balance: Nat,
        liquidity_pool_icp_balance: Nat,
    ) -> ExchangeAmounts {
        let sats_rate = Nat::from((icp_btc_rate * 100_000_000_f64) as u64);
        // convert the ICP to Sats
        let liquidity_pool_btc_balance =
            (liquidity_pool_icp_balance.clone() * sats_rate.clone()) / 10_u32.pow(8);
        println!("liquidity_pool_btc_balance: {}", liquidity_pool_btc_balance);

        // get the amount to exchange
        // get the minimum between the allowance, the swap account BTC balance and the liquidity pool ICP balance (expressed in sats)
        let sats_to_send_to_liquidity_pool = swap_account_balance
            .min(allowance)
            .min(liquidity_pool_btc_balance);
        // convert to ICP
        let icp_to_send_to_swap_account =
            sats_to_send_to_liquidity_pool.clone() * 10_u32.pow(8) / sats_rate;

        ExchangeAmounts {
            icp: icp_to_send_to_swap_account,
            btc: sats_to_send_to_liquidity_pool,
        }
    }
}

struct ExchangeAmounts {
    icp: Nat,
    /// Sats
    btc: Nat,
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_init_and_get_accounts() {
        LiquidityPool::init();
        let account = LiquidityPool::accounts();
        assert_eq!(account.ckbtc.owner, utils::id());
        assert_eq!(account.icp.owner, utils::id());
        assert_eq!(
            account.ckbtc,
            CKBTC_ACCOUNT.with_borrow(|account| account.get().clone()).0
        );
        assert_eq!(
            account.icp,
            ICP_ACCOUNT.with_borrow(|account| account.get().clone()).0
        );
    }

    #[tokio::test]
    async fn test_should_get_balance() {
        LiquidityPool::init();
        let balance = LiquidityPool::balance().await.unwrap();
        assert_eq!(balance.ckbtc, 888010101000000u64);
        assert_eq!(balance.icp, 888010101000000u64);
    }

    #[test]
    fn test_should_calculate_the_exchange_amounts_icp_lt_btc() {
        let icp_btc_rate = 0.00021543;
        let swap_balance: Nat = 5_299_287_u64.into(); // about 2245$
        let allowance: Nat = 5_000_000_u64.into();
        let icp_balance: Nat = 716_774_022_u64.into(); // about 65$

        // get amounts
        let amounts =
            LiquidityPool::get_exchange_amounts(icp_btc_rate, allowance, swap_balance, icp_balance);

        assert_eq!(amounts.btc, 154_414_u64);
        assert_eq!(amounts.icp, 716_771_108_u64);
    }

    #[test]
    fn test_should_calculate_the_exchange_amounts_icp_gt_btc() {
        let icp_btc_rate = 0.00021543;
        let swap_balance: Nat = 5_299_287_u64.into(); // about 2245$
        let allowance: Nat = 50_000_u64.into(); // about 40$
        let icp_balance: Nat = 716_774_022_u64.into(); // about 65$

        // get amounts
        let amounts =
            LiquidityPool::get_exchange_amounts(icp_btc_rate, allowance, swap_balance, icp_balance);

        assert_eq!(amounts.btc, 50_000_u64);
        assert_eq!(amounts.icp, 232_093_951_u64);
    }
}
