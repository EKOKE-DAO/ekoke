mod swap_fee;
mod swap_pool;

use did::fly::{FlyError, FlyResult, PicoFly};
use did::H160;
use icrc::icrc1::account::Account;

use self::swap_fee::SwapFee;
use self::swap_pool::SwapPool;
use crate::app::{Balance, Configuration};
use crate::utils;

/// ERC20 Bridge
pub struct Erc20Bridge;

impl Erc20Bridge {
    /// Swaps the ERC20 token to the FLY token.
    pub async fn swap(recipient: H160, amount: PicoFly) -> FlyResult<()> {
        todo!();
    }

    /// Returns the swap fee.
    pub fn get_swap_fee() -> u64 {
        SwapFee::get_swap_fee()
    }

    /// Sets the swap fee.
    pub fn set_swap_fee(swap_fee: u64) -> FlyResult<()> {
        SwapFee::set_swap_fee(swap_fee)
    }
}
