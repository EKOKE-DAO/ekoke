mod eth_wallet;
mod swap_fee;
mod swap_pool;

use did::fly::{FlyError, FlyResult, PicoFly};
use did::H160;
use icrc::icrc1::account::Account;

use self::eth_wallet::EthWallet;
use self::swap_fee::SwapFee;
use self::swap_pool::SwapPool;
use crate::app::{Balance, Configuration};
use crate::utils;

/// ERC20 Bridge
pub struct Erc20Bridge;

impl Erc20Bridge {
    /// Swaps the ICRC FLY token to the ERC20 FLY token.
    pub async fn swap_icrc_to_erc20(recipient: H160, amount: PicoFly) -> FlyResult<()> {
        todo!();
    }

    /// Swaps the ERC20 FLY token to the ICRC FLY token.
    ///
    /// This method is easier than the conversion from ICRC FLY to ERC20 FLY and it basically
    /// just transfer the amount from the swap pool to the recipient.
    pub async fn swap_erc20_to_icrc(recipient: Account, amount: PicoFly) -> FlyResult<()> {
        SwapPool::withdraw(recipient, amount).await
    }

    /// Returns the swap fee.
    pub fn get_swap_fee() -> u64 {
        SwapFee::get_swap_fee()
    }

    /// Sets the swap fee.
    pub fn set_swap_fee(swap_fee: u64) -> FlyResult<()> {
        SwapFee::set_swap_fee(swap_fee)
    }

    /// Returns the address of the ETH wallet
    pub async fn get_wallet_address() -> FlyResult<H160> {
        EthWallet::address().await
    }
}
