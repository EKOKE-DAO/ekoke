mod eth_rpc;
mod eth_wallet;
mod swap_fee;
mod swap_pool;

use did::ekoke::{BalanceError, EkokeError, EkokeResult, PicoEkoke};
use did::H160;
use icrc::icrc1::account::Account;

use self::eth_rpc::EthRpcClient;
use self::eth_wallet::EthWallet;
use self::swap_fee::SwapFee;
use self::swap_pool::SwapPool;
use super::balance::Balance;
use super::configuration::Configuration;

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
        // check caller balance
        if Balance::balance_of(caller)? < amount {
            return Err(EkokeError::Balance(BalanceError::InsufficientBalance));
        }

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

    /// Swaps the ERC20 FLY token to the ICRC FLY token.
    ///
    /// This method is easier than the conversion from ICRC FLY to ERC20 FLY and it basically
    /// just transfer the amount from the swap pool to the recipient.
    pub async fn swap_erc20_to_icrc(recipient: Account, amount: PicoEkoke) -> EkokeResult<()> {
        SwapPool::withdraw(recipient, amount).await
    }

    /// Returns the swap fee.
    pub fn get_swap_fee() -> u64 {
        SwapFee::get_swap_fee()
    }

    /// Sets the swap fee.
    pub fn set_swap_fee(swap_fee: u64) -> EkokeResult<()> {
        SwapFee::set_swap_fee(swap_fee)
    }

    /// Returns the address of the ETH wallet
    pub async fn get_wallet_address() -> EkokeResult<H160> {
        EthWallet::address().await
    }
}
