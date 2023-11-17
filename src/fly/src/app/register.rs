use std::cell::RefCell;

use candid::Nat;
use did::fly::{FlyError, FlyResult, RegisterError, Transaction};
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
    pub fn insert_tx(tx: Transaction) -> FlyResult<Nat> {
        REGISTER.with_borrow_mut(|register| {
            let id = register.len();
            register.push(&tx).map_err(|_| FlyError::StorageError)?;

            Ok(id.into())
        })
    }

    /// Get a transaction from the register by its ID
    pub fn get_tx(id: u64) -> FlyResult<Transaction> {
        REGISTER.with_borrow(|register| {
            register
                .get(id)
                .ok_or(FlyError::Register(RegisterError::TransactionNotFound))
        })
    }
}

#[cfg(test)]
mod test {

    use icrc::icrc1::transfer::Memo;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{alice_account, bob_account};
    use crate::constants::ICRC1_FEE;
    use crate::utils::fly_to_picofly;

    #[test]
    fn test_should_insert_tx() {
        let tx = Transaction {
            from: alice_account(),
            to: bob_account(),
            amount: fly_to_picofly(50),
            fee: ICRC1_FEE,
            memo: None,
            created_at: crate::utils::time(),
        };
        assert_eq!(Register::insert_tx(tx).unwrap(), Nat::from(0));
        assert!(Register::get_tx(0).is_ok());

        let tx = Transaction {
            from: alice_account(),
            to: bob_account(),
            amount: fly_to_picofly(50),
            fee: ICRC1_FEE,
            memo: Some(Memo::from(
                "12341235412523524353451234123541".as_bytes().to_vec(),
            )),
            created_at: crate::utils::time(),
        };
        assert_eq!(Register::insert_tx(tx).unwrap(), Nat::from(1));
        assert!(Register::get_tx(1).is_ok());

        assert!(Register::get_tx(2).is_err());
    }
}
