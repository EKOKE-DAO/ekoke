use std::cell::RefCell;
use std::collections::HashSet;

use candid::{Nat, Principal};
use did::ekoke_index::{
    GetTransactions, GetTransactionsErr, GetTransactionsResult, Transaction, TransactionWithId,
    TxId,
};
use did::StorableAccount;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use icrc::icrc1::account::{Account, Subaccount};
use num_traits::ToPrimitive as _;

use super::archive_client::ArchiveCanister;
use crate::app::memory::{
    ACCOUNTS_FROM_MEMORY_ID, ACCOUNTS_SPENDER_MEMORY_ID, ACCOUNTS_TO_MEMORY_ID, MEMORY_MANAGER,
};

thread_local! {
    static TRANSACTIONS_FROM_ACCOUNTS: RefCell<StableBTreeMap<u64, StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(ACCOUNTS_FROM_MEMORY_ID)))
    );

    static TRANSACTIONS_TO_ACCOUNTS: RefCell<StableBTreeMap<u64, StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(ACCOUNTS_TO_MEMORY_ID)))
    );

    static TRANSACTIONS_SPENDER_ACCOUNTS: RefCell<StableBTreeMap<u64, StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(ACCOUNTS_SPENDER_MEMORY_ID)))
    );
}

/// The register contains the transactions history
pub struct Index;

impl Index {
    /// Insert a transaction in the index.
    pub fn commit(id: u64, tx: Transaction) -> TxId {
        // insert accounts
        if let Some(from) = tx.from() {
            TRANSACTIONS_FROM_ACCOUNTS.with_borrow_mut(|from_accounts| {
                from_accounts.insert(id, from.into());
            });
        }
        if let Some(to) = tx.to() {
            TRANSACTIONS_TO_ACCOUNTS.with_borrow_mut(|to_accounts| {
                to_accounts.insert(id, to.into());
            });
        }
        if let Some(spender) = tx.spender() {
            TRANSACTIONS_SPENDER_ACCOUNTS.with_borrow_mut(|spender_accounts| {
                spender_accounts.insert(id, spender.into());
            });
        }

        id.into()
    }

    /// List subaccounts associated to a principal.
    /// If start is provided, the list will start from the subaccount after the provided one.
    pub fn list_subaccounts(owner: Principal, start: Option<Subaccount>) -> Vec<Subaccount> {
        let mut subaccounts = HashSet::new();
        // list all transactions with owner
        let mut collect_accounts_fn =
            |accounts: &StableBTreeMap<u64, StorableAccount, VirtualMemory<DefaultMemoryImpl>>| {
                for (_, account) in accounts.iter() {
                    if account.0.owner == owner {
                        if let Some(subaccount) = account.0.subaccount {
                            subaccounts.insert(subaccount);
                        }
                    }
                }
            };
        TRANSACTIONS_FROM_ACCOUNTS.with_borrow(&mut collect_accounts_fn);
        TRANSACTIONS_TO_ACCOUNTS.with_borrow(&mut collect_accounts_fn);
        TRANSACTIONS_SPENDER_ACCOUNTS.with_borrow(collect_accounts_fn);

        let mut subaccounts = subaccounts.into_iter().collect::<Vec<_>>();
        if let Some(start) = start {
            let index = subaccounts
                .iter()
                .position(|sa| *sa == start)
                .unwrap_or_default();
            subaccounts = subaccounts.split_off(index + 1);
        }

        subaccounts
    }

    /// Get transactions for an account
    pub async fn get_account_transactions(
        account: Account,
        start: Option<TxId>,
        max_results: Nat,
    ) -> GetTransactionsResult {
        let max_results = max_results.0.to_usize().unwrap_or_default();
        let mut transaction_ids = Vec::with_capacity(max_results);
        let start = start.map(|s| s.0.to_u64().unwrap_or_default());
        let mut oldest_tx_id = None;
        // search for transactions
        TRANSACTIONS_FROM_ACCOUNTS.with_borrow(|from_accounts| {
            for (id, acc) in from_accounts.iter() {
                if acc.0 == account {
                    transaction_ids.push(id);
                }
            }
        });
        TRANSACTIONS_TO_ACCOUNTS.with_borrow(|to_accounts| {
            for (id, acc) in to_accounts.iter() {
                if acc.0 == account {
                    transaction_ids.push(id);
                }
            }
        });
        TRANSACTIONS_SPENDER_ACCOUNTS.with_borrow(|spender_accounts| {
            for (id, acc) in spender_accounts.iter() {
                if acc.0 == account {
                    transaction_ids.push(id);
                }
            }
        });

        // sort transactions
        transaction_ids.sort();
        // remove duplicates
        transaction_ids.dedup();
        // if start is provided, remove transactions before start
        if let Some(start) = start {
            let index = transaction_ids
                .iter()
                .position(|id| *id == start)
                .unwrap_or_default();
            transaction_ids = transaction_ids.split_off(index);
        }
        // if max_results is provided, limit the number of transactions
        if transaction_ids.len() > max_results {
            transaction_ids.truncate(max_results);
        }

        // get oldest tx id
        if let Some(id) = transaction_ids.first() {
            oldest_tx_id = Some((*id).into());
        }

        // collect transactions
        let mut transactions = Vec::with_capacity(transaction_ids.len());
        for id in transaction_ids {
            if let Some(tx) = ArchiveCanister::get_transaction(id)
                .await
                .map_err(|e| GetTransactionsErr { message: e })?
            {
                transactions.push(TransactionWithId {
                    id: id.into(),
                    transaction: tx,
                });
            }
        }

        Ok(GetTransactions {
            oldest_tx_id,
            transactions,
        })
    }
}

#[cfg(test)]
mod test {
    use did::ekoke_index::Transfer;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{
        alice, alice_account, bob, bob_account, charlie_account, random_alice_account,
    };

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

        let tx_id = Index::commit(0, tx.clone());
        assert_eq!(tx_id, 0u64);

        // check tx
        let spender = TRANSACTIONS_SPENDER_ACCOUNTS
            .with_borrow(|spender_accounts| spender_accounts.get(&0u64).unwrap().0);
        assert_eq!(spender, charlie_account());

        let spender = TRANSACTIONS_TO_ACCOUNTS
            .with_borrow(|spender_accounts| spender_accounts.get(&0u64).unwrap().0);
        assert_eq!(spender, bob_account());

        let spender = TRANSACTIONS_FROM_ACCOUNTS
            .with_borrow(|spender_accounts| spender_accounts.get(&0u64).unwrap().0);
        assert_eq!(spender, alice_account());
    }

    #[test]
    fn test_should_get_subaccounts() {
        for iter in 0..100u64 {
            let tx = Transaction {
                kind: "transfer".to_string(),
                mint: None,
                burn: None,
                transfer: Some(Transfer {
                    amount: 100_u64.into(),
                    from: random_alice_account(),
                    to: bob_account(),
                    spender: None,
                    memo: None,
                    created_at_time: None,
                    fee: None,
                }),
                approve: None,
                timestamp: 0,
            };
            let tx_id = Index::commit(iter, tx.clone());
            assert_eq!(tx_id, iter);
        }

        // get subaccounts for alice
        let subaccounts = Index::list_subaccounts(alice(), None);
        assert_eq!(subaccounts.len(), 100);

        // get subaccounts for bob
        let subaccounts = Index::list_subaccounts(bob(), None);
        assert_eq!(subaccounts.len(), 1);
    }

    #[tokio::test]
    async fn test_should_get_transactions() {
        for iter in 0..100u64 {
            let tx = Transaction {
                kind: "transfer".to_string(),
                mint: None,
                burn: None,
                transfer: Some(Transfer {
                    amount: 100_u64.into(),
                    from: random_alice_account(),
                    to: bob_account(),
                    spender: None,
                    memo: None,
                    created_at_time: None,
                    fee: None,
                }),
                approve: None,
                timestamp: 0,
            };
            let tx_id = Index::commit(iter, tx.clone());
            assert_eq!(tx_id, iter);
        }

        let account_txs = Index::get_account_transactions(bob_account(), None, 100u64.into())
            .await
            .unwrap();
        assert_eq!(account_txs.transactions.len(), 100);
        assert_eq!(account_txs.oldest_tx_id, Some(0u64.into()));

        let account_txs = Index::get_account_transactions(bob_account(), None, 10_u64.into())
            .await
            .unwrap();
        assert_eq!(account_txs.transactions.len(), 10);
        assert_eq!(account_txs.oldest_tx_id, Some(0u64.into()));

        let account_txs =
            Index::get_account_transactions(bob_account(), Some(1u64.into()), 10_u64.into())
                .await
                .unwrap();
        assert_eq!(account_txs.transactions.len(), 10);
        assert_eq!(account_txs.oldest_tx_id, Some(1u64.into()));
    }
}
