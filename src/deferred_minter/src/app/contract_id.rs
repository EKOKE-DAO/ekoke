use std::cell::RefCell;

use did::deferred::{DeferredMinterError, DeferredMinterResult};
use did::ID;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{MEMORY_MANAGER, NEXT_CONTRACT_ID_MEMORY_ID};

thread_local! {

    /// gas price
    static NEXT_CONTRACT_ID: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(NEXT_CONTRACT_ID_MEMORY_ID)), 1).unwrap()
    );


}

pub struct ContractId;

impl ContractId {
    pub fn incr_next_contract_id() -> DeferredMinterResult<()> {
        NEXT_CONTRACT_ID.with_borrow_mut(|cell| {
            let next_id = *cell.get() + 1;
            cell.set(next_id)
                .map_err(|_| DeferredMinterError::StorageError)
        })?;

        Ok(())
    }

    pub fn get_next_contract_id() -> ID {
        NEXT_CONTRACT_ID.with_borrow(|cell| (*cell.get()).into())
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_get_and_incr_contract_id() {
        assert_eq!(ContractId::get_next_contract_id(), ID::from(1u64));
        assert!(ContractId::incr_next_contract_id().is_ok());
        assert_eq!(ContractId::get_next_contract_id(), ID::from(2u64));
    }
}
