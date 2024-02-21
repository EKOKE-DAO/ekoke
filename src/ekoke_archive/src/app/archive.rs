use std::cell::RefCell;

use did::ekoke_archive::Transaction;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};

use crate::app::memory::{MEMORY_MANAGER, TRANSACTIONS_MEMORY_ID};

thread_local! {
    static TRANSACTIONS: RefCell<StableBTreeMap<u64, Transaction, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(TRANSACTIONS_MEMORY_ID))));

}

/// The register contains the transactions history
pub struct Archive;

impl Archive {
    /// Insert a transaction in the index.
    pub fn commit(tx: Transaction) {
        TRANSACTIONS.with_borrow_mut(|transactions| {
            let id = transactions.len();

            // insert transaction
            transactions.insert(id, tx);
        })
    }

    pub fn get_transaction(id: u64) -> Option<Transaction> {
        TRANSACTIONS.with_borrow(|transactions| transactions.get(&id))
    }

    pub fn get_transactions(start: u64, length: u64) -> Vec<Transaction> {
        TRANSACTIONS.with_borrow(|transactions| {
            transactions
                .range(start..)
                .take(length as usize)
                .map(|(_, tx)| tx.clone())
                .collect()
        })
    }
}

#[cfg(test)]
mod test {
    use did::ekoke_index::Transfer;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{alice_account, bob_account, charlie_account};

    #[test]
    fn test_should_commit_transaction() {
        let tx = Transaction {
            kind: "transfer".to_string(),
            mint: None,
            burn: None,
            transfer: Some(Transfer {
                amount: 100_u64.into(),
                from: alice_account(),
                to: bob_account(),
                spender: Some(charlie_account()),
                memo: None,
                created_at_time: None,
                fee: None,
            }),
            approve: None,
            timestamp: 0,
        };

        Archive::commit(tx.clone());

        let transactions = TRANSACTIONS.with_borrow(|transactions| transactions.len());
        assert_eq!(transactions, 1);
    }

    #[test]
    fn test_should_get_transaction() {
        let tx = Transaction {
            kind: "transfer".to_string(),
            mint: None,
            burn: None,
            transfer: Some(Transfer {
                amount: 100_u64.into(),
                from: alice_account(),
                to: bob_account(),
                spender: Some(charlie_account()),
                memo: None,
                created_at_time: None,
                fee: None,
            }),
            approve: None,
            timestamp: 0,
        };

        Archive::commit(tx.clone());

        let transaction = Archive::get_transaction(0);
        assert_eq!(transaction, Some(tx));
    }

    #[test]
    fn test_should_get_transactions() {
        let tx1 = Transaction {
            kind: "transfer".to_string(),
            mint: None,
            burn: None,
            transfer: Some(Transfer {
                amount: 100_u64.into(),
                from: alice_account(),
                to: bob_account(),
                spender: Some(charlie_account()),
                memo: None,
                created_at_time: None,
                fee: None,
            }),
            approve: None,
            timestamp: 0,
        };

        let tx2 = Transaction {
            kind: "transfer".to_string(),
            mint: None,
            burn: None,
            transfer: Some(Transfer {
                amount: 100_u64.into(),
                from: alice_account(),
                to: bob_account(),
                spender: Some(charlie_account()),
                memo: None,
                created_at_time: None,
                fee: None,
            }),
            approve: None,
            timestamp: 0,
        };

        Archive::commit(tx1.clone());
        Archive::commit(tx2.clone());

        let transactions = Archive::get_transactions(1, 1);
        assert_eq!(transactions, vec![tx2]);
    }
}
