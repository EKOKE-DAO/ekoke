use std::cell::RefCell;

use candid::Nat;
use did::ekoke::{EkokeError, EkokeResult, Transaction};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableVec};

use crate::app::memory::{MEMORY_MANAGER, REGISTER_MEMORY_ID};

thread_local! {
    static REGISTER: RefCell<StableVec<Transaction, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableVec::new(MEMORY_MANAGER.with(|mm| mm.get(REGISTER_MEMORY_ID))).unwrap());
}

/// The register contains the transactions history
pub struct Register;

impl Register {
    /// Insert a transaction in the register.
    /// Returns the transaction ID
    pub fn insert_tx(tx: Transaction) -> EkokeResult<Nat> {
        REGISTER.with_borrow_mut(|register| {
            let id = register.len();
            register.push(&tx).map_err(|_| EkokeError::StorageError)?;

            Ok(id.into())
        })
    }
}
