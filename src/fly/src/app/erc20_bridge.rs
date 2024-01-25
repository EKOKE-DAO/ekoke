mod swap_fee;

use did::fly::FlyResult;

use self::swap_fee::SwapFee;

/// ERC20 Bridge
pub struct Erc20Bridge;

impl Erc20Bridge {
    /// Returns the swap fee.
    pub fn get_swap_fee() -> u64 {
        SwapFee::get_swap_fee()
    }

    /// Sets the swap fee.
    pub fn set_swap_fee(swap_fee: u64) -> FlyResult<()> {
        SwapFee::set_swap_fee(swap_fee)
    }
}
