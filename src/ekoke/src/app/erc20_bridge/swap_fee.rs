use std::cell::RefCell;

use did::ekoke::{EkokeError, EkokeResult};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{
    ERC20_GAS_PRICE_MEMORY_ID, ERC20_LAST_GAS_PRICE_UPDATE_MEMORY_ID, MEMORY_MANAGER,
};
use crate::constants::{THREE_HOURS, TRANSCRIBE_SWAP_TX_GAS};
use crate::utils::time;

thread_local! {

    static GAS_PRICE: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableCell::new(
                MEMORY_MANAGER.with(|mm| mm.get(ERC20_GAS_PRICE_MEMORY_ID)),
                0_u64,
            ).unwrap()
        );

    static LAST_GAS_PRICE_UPDATE : RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableCell::new(
                MEMORY_MANAGER.with(|mm| mm.get(ERC20_LAST_GAS_PRICE_UPDATE_MEMORY_ID)),
                0_u64,
            ).unwrap()
        );

}

/// Swap fee for ERC20 token swaps.
pub struct SwapFee;

impl SwapFee {
    /// Returns the swap fee.
    pub fn get_swap_fee() -> u64 {
        Self::get_gas_price() * TRANSCRIBE_SWAP_TX_GAS
    }

    /// Returns the gas price.
    pub fn get_gas_price() -> u64 {
        GAS_PRICE.with(|sf| *sf.borrow().get())
    }

    /// Sets the swap fee and update last swap fee update.
    pub fn set_gas_price(gas_price: u64) -> EkokeResult<()> {
        GAS_PRICE
            .with_borrow_mut(|sf| sf.set(gas_price))
            .map_err(|_| EkokeError::StorageError)?;

        LAST_GAS_PRICE_UPDATE
            .with_borrow_mut(|lsfu| lsfu.set(time()).map_err(|_| EkokeError::StorageError))?;

        Ok(())
    }

    /// Returns whether the swap fee should be updated.
    /// The gas price should be updated every three hours.
    pub fn should_update_gas_price() -> bool {
        LAST_GAS_PRICE_UPDATE
            .with(|lsfu| time() - *lsfu.borrow().get() > THREE_HOURS.as_nanos() as u64)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::constants::TRANSCRIBE_SWAP_TX_GAS;

    #[test]
    fn test_swap_fee() {
        assert_eq!(SwapFee::get_swap_fee(), 0);
        assert_eq!(LAST_GAS_PRICE_UPDATE.with(|lsfu| *lsfu.borrow().get()), 0);

        SwapFee::set_gas_price(100).unwrap();
        assert_eq!(SwapFee::get_gas_price(), 100);
        assert_eq!(SwapFee::get_swap_fee(), 100 * TRANSCRIBE_SWAP_TX_GAS);

        SwapFee::set_gas_price(200).unwrap();
        assert_eq!(SwapFee::get_gas_price(), 200);
        assert_eq!(SwapFee::get_swap_fee(), 200 * TRANSCRIBE_SWAP_TX_GAS);

        assert_ne!(LAST_GAS_PRICE_UPDATE.with(|lsfu| *lsfu.borrow().get()), 0);
    }

    #[test]
    fn test_should_tell_whether_to_update_gas_price() {
        // three hours elapsed
        SwapFee::set_gas_price(130).unwrap();
        LAST_GAS_PRICE_UPDATE
            .with_borrow_mut(|lsfu| lsfu.set(time() - THREE_HOURS.as_nanos() as u64))
            .unwrap();
        assert!(SwapFee::should_update_gas_price());

        // time has not elapsed
        SwapFee::set_gas_price(130).unwrap();
        assert!(!SwapFee::should_update_gas_price());
    }
}
