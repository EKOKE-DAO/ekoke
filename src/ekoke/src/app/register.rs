use std::cell::RefCell;

use candid::Nat;
use did::ekoke::{EkokeError, EkokeResult, RegisterError, Transaction};
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

    /// Get a transaction from the register by its ID
    pub fn get_tx(id: u64) -> EkokeResult<Transaction> {
        REGISTER.with_borrow(|register| {
            register
                .get(id)
                .ok_or(EkokeError::Register(RegisterError::TransactionNotFound))
        })
    }
}

#[cfg(test)]
mod test {

    use icrc::icrc1::transfer::Memo;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{alice_account, bob_account, ekoke_to_picoekoke};
    use crate::constants::ICRC1_FEE;

    #[test]
    fn test_should_insert_tx() {
        let tx = Transaction {
            from: alice_account(),
            to: bob_account(),
            amount: ekoke_to_picoekoke(50),
            fee: ICRC1_FEE.into(),
            memo: None,
            created_at: crate::utils::time(),
        };
        assert_eq!(Register::insert_tx(tx).unwrap(), Nat::from(0));
        assert!(Register::get_tx(0).is_ok());

        let tx = Transaction {
            from: alice_account(),
            to: bob_account(),
            amount: ekoke_to_picoekoke(50),
            fee: ICRC1_FEE.into(),
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
