mod cketh_withdrawal;
mod eth_rpc;
mod eth_wallet;
mod gas_station;
mod swap_fee;
mod swap_pool;

use std::cell::RefCell;

use did::ekoke::{AllowanceError, BalanceError, EkokeError, EkokeResult, PicoEkoke};
use did::H160;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use icrc::icrc1::account::Account;
use icrc::IcrcLedgerClient;

use self::eth_rpc::EthRpcClient;
use self::eth_wallet::EthWallet;
use self::gas_station::GasStation;
use self::swap_fee::SwapFee;
use self::swap_pool::SwapPool;
use super::configuration::Configuration;
use crate::app::memory::{ERC20_LOGS_START_BLOCK_MEMORY_ID, MEMORY_MANAGER};
use crate::constants::ERC20_SWAP_FEE_INTEREST;
use crate::utils::id;

thread_local! {
    /// ERC20 logs start block
    static ERC20_LOGS_START_BLOCK: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableCell::new(
                MEMORY_MANAGER.with(|mm| mm.get(ERC20_LOGS_START_BLOCK_MEMORY_ID)),
                0,
            ).unwrap()
        );
}

/// ERC20 Bridge
pub struct Erc20Bridge;

impl Erc20Bridge {
    /// Swaps the ICRC FLY token to the ERC20 FLY token.
    ///
    /// This function won't check:
    ///
    /// - ckETH allowance
    ///
    /// This function will just:
    ///
    /// - call swap on the ERC20 contract
    /// - transfer the amount from the caller to the swap pool
    pub async fn swap_icrc_to_erc20(
        caller: Account,
        recipient: H160,
        amount: PicoEkoke,
    ) -> EkokeResult<String> {
        // check allowance
        Self::check_balance_and_allowance(caller, amount.clone()).await?;

        let rpc_client = EthRpcClient::new(Configuration::get_eth_network());
        // make transaction
        let transaction = rpc_client
            .ekoke_transcribe_swap_tx(EthWallet::address().await?, recipient, amount.clone())
            .await?;
        // sign transaction
        let signed_tx = EthWallet::sign_transaction(transaction).await?;
        // send transaction
        let hash = rpc_client.send_tx(signed_tx).await?;

        // deposit to swap pool
        SwapPool::deposit(caller, amount).await?;

        Ok(hash)
    }

    /// Get the Ekoke Swap event and then
    ///
    /// Swaps the ERC20 FLY token to the ICRC FLY token.
    ///
    /// This method is easier than the conversion from ICRC FLY to ERC20 FLY and it basically
    /// just transfer the amount from the swap pool to the recipient.
    #[allow(dead_code)]
    pub async fn swap_erc20_to_icrc() -> EkokeResult<()> {
        let rpc_client = EthRpcClient::new(Configuration::get_eth_network());
        // get ekoke swapped events
        let from_block = ERC20_LOGS_START_BLOCK.with(|cell| *cell.borrow().get());
        let (last_block, events) = rpc_client.get_ekoke_swapped_events(from_block).await?;

        // update last block
        ERC20_LOGS_START_BLOCK
            .with(|cell| cell.borrow_mut().set(last_block + 1))
            .unwrap();

        // withdraw to recipient
        for event in events {
            match (event.principal(), event.amount()) {
                (Ok(principal), Ok(amount)) => {
                    SwapPool::withdraw(Account::from(principal), amount.into()).await?;
                }
                _ => continue,
            }
        }

        Ok(())
    }

    /// Returns the swap fee.
    pub fn get_swap_fee() -> u64 {
        SwapFee::get_swap_fee()
    }

    /// Sets the gas price.
    pub fn set_gas_price(gas_price: u64) -> EkokeResult<()> {
        SwapFee::set_gas_price(gas_price)
    }

    /// Returns the address of the ETH wallet
    pub async fn get_wallet_address() -> EkokeResult<H160> {
        EthWallet::address().await
    }

    /// Fetches the current gas price from etherscan
    ///
    /// Used only in wasm32
    #[allow(dead_code)]
    pub async fn fetch_gas_price() -> EkokeResult<()> {
        if !SwapFee::should_update_gas_price() {
            return Ok(());
        }
        // fetch gas price and add 10%
        let gas_price =
            (GasStation::fetch_gas_price().await? as f64 * ERC20_SWAP_FEE_INTEREST) as u64;
        // update price
        Self::set_gas_price(gas_price)?;

        Ok(())
    }

    /// Withdraws current ckETH balance converting it to ETH and sending them to the ETH canister wallet.
    #[allow(dead_code)]
    pub async fn withdraw_cketh_to_eth() -> EkokeResult<()> {
        cketh_withdrawal::CkEthWithdrawal::withdraw_cketh().await
    }

    async fn check_balance_and_allowance(caller: Account, amount: PicoEkoke) -> EkokeResult<()> {
        let ledger_client = IcrcLedgerClient::new(Configuration::get_ledger_canister());
        let caller_balance = ledger_client
            .icrc1_balance_of(caller)
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?;
        // check allowance
        let fee = ledger_client
            .icrc1_fee()
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?;
        // check caller balance
        if caller_balance < amount {
            return Err(EkokeError::Balance(BalanceError::InsufficientBalance));
        }
        let total_amount = amount + fee;
        let allowance = ledger_client
            .icrc2_allowance(Self::swap_account(), caller)
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?
            .allowance;

        if allowance < total_amount {
            return Err(EkokeError::Allowance(AllowanceError::InsufficientFunds));
        }

        Ok(())
    }

    #[inline]
    fn swap_account() -> Account {
        Account {
            owner: id(),
            subaccount: None,
        }
    }
}
