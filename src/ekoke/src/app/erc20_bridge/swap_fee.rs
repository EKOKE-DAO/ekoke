use std::cell::RefCell;

use did::ekoke::{EkokeError, EkokeResult};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{
    ERC20_SWAP_FEE_LAST_UPDATE_MEMORY_ID, ERC20_SWAP_FEE_MEMORY_ID, MEMORY_MANAGER,
};
use crate::constants::{ERC20_SWAP_FEE_MULTIPLIER, ONE_WEEK};
use crate::utils::time;

thread_local! {

    static SWAP_FEE: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableCell::new(
                MEMORY_MANAGER.with(|mm| mm.get(ERC20_SWAP_FEE_MEMORY_ID)),
                0_u64,
            ).unwrap()
        );

    static LAST_SWAP_FEE_UPDATE : RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableCell::new(
                MEMORY_MANAGER.with(|mm| mm.get(ERC20_SWAP_FEE_LAST_UPDATE_MEMORY_ID)),
                0_u64,
            ).unwrap()
        );

}

/// Swap fee for ERC20 token swaps.
pub struct SwapFee;

impl SwapFee {
    /// Returns the swap fee.
    pub fn get_swap_fee() -> u64 {
        SWAP_FEE.with(|sf| *sf.borrow().get())
    }

    /// Sets the swap fee and update last swap fee update.
    pub fn set_swap_fee(swap_fee: u64) -> EkokeResult<()> {
        SWAP_FEE
            .with_borrow_mut(|sf| sf.set(swap_fee))
            .map_err(|_| EkokeError::StorageError)?;

        LAST_SWAP_FEE_UPDATE
            .with_borrow_mut(|lsfu| lsfu.set(time()).map_err(|_| EkokeError::StorageError))?;

        Ok(())
    }

    /// Returns whether the swap fee should be updated.
    pub fn should_update_swap_fee(paid_gas: u64) -> bool {
        let current_real_swap_fee = Self::get_real_swap_fee();
        let paid_gas = paid_gas as f64;

        let has_elapsed_one_week = LAST_SWAP_FEE_UPDATE
            .with(|lsfu| time() - *lsfu.borrow().get() > ONE_WEEK.as_nanos() as u64);

        (current_real_swap_fee * 1.75 <= paid_gas)
            || (current_real_swap_fee * 1.25 <= paid_gas && has_elapsed_one_week)
            || (current_real_swap_fee * 0.75 >= paid_gas && has_elapsed_one_week)
    }

    /// Get the swap fee without the multiplier.
    fn get_real_swap_fee() -> f64 {
        SWAP_FEE.with(|sf| *sf.borrow().get()) as f64 / ERC20_SWAP_FEE_MULTIPLIER
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn test_swap_fee() {
        assert_eq!(SwapFee::get_swap_fee(), 0);
        assert_eq!(LAST_SWAP_FEE_UPDATE.with(|lsfu| *lsfu.borrow().get()), 0);

        SwapFee::set_swap_fee(100).unwrap();
        assert_eq!(SwapFee::get_swap_fee(), 100);

        SwapFee::set_swap_fee(200).unwrap();
        assert_eq!(SwapFee::get_swap_fee(), 200);

        assert_ne!(LAST_SWAP_FEE_UPDATE.with(|lsfu| *lsfu.borrow().get()), 0);
    }

    #[test]
    fn test_should_tell_whether_to_update_swap_fee() {
        // init swap fee to 120 (real fee is 100)
        SwapFee::set_swap_fee(130).unwrap();
        assert_eq!(SwapFee::get_real_swap_fee(), 100.0);

        // 75%
        assert!(SwapFee::should_update_swap_fee(175));

        // 25 % and one week has elapsed
        SwapFee::set_swap_fee(130).unwrap();
        LAST_SWAP_FEE_UPDATE
            .with_borrow_mut(|lsfu| lsfu.set(time() - ONE_WEEK.as_nanos() as u64))
            .unwrap();
        assert!(SwapFee::should_update_swap_fee(125));

        // 25% and one week has not elapsed
        SwapFee::set_swap_fee(130).unwrap();
        assert!(!SwapFee::should_update_swap_fee(125));

        // 0.75 and one week has elapsed
        SwapFee::set_swap_fee(130).unwrap();
        LAST_SWAP_FEE_UPDATE
            .with_borrow_mut(|lsfu| lsfu.set(time() - ONE_WEEK.as_nanos() as u64))
            .unwrap();
        assert!(SwapFee::should_update_swap_fee(75));

        // 0.75 and one week has not elapsed
        SwapFee::set_swap_fee(130).unwrap();
        assert!(!SwapFee::should_update_swap_fee(75));
    }
}
