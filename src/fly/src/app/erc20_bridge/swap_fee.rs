use std::cell::RefCell;

use did::fly::{FlyError, FlyResult};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{
    ERC20_SWAP_FEE_LAST_UPDATE_MEMORY_ID, ERC20_SWAP_FEE_MEMORY_ID, MEMORY_MANAGER,
};
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
    pub fn set_swap_fee(swap_fee: u64) -> FlyResult<()> {
        SWAP_FEE
            .with_borrow_mut(|sf| sf.set(swap_fee))
            .map_err(|_| FlyError::StorageError)?;

        LAST_SWAP_FEE_UPDATE
            .with_borrow_mut(|lsfu| lsfu.set(time()).map_err(|_| FlyError::StorageError))?;

        Ok(())
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
}
