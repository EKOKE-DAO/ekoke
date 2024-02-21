mod archive_client;
mod configuration;
mod index;
mod inspect;
mod memory;
#[cfg(test)]
mod test_utils;

use candid::Principal;
use did::ekoke_index::{
    EkokeIndexInitData, GetAccountTransactionArgs, GetTransactionsResult, ListSubaccountsArgs,
    Transaction, TxId,
};
use icrc::icrc1::account::Subaccount;

use self::configuration::Configuration;
use self::index::Index;
pub use self::inspect::Inspect;
use crate::utils::caller;

pub struct EkokeIndexCanister;

impl EkokeIndexCanister {
    pub fn init(args: EkokeIndexInitData) {
        Configuration::set_archive_canister(args.archive_id);
        Configuration::set_ledger_canister(args.ledger_id);
    }

    /// Get ledger canister id
    pub fn ledger_id() -> Principal {
        Configuration::get_ledger_canister()
    }

    /// List subaccounts associated to a principal.
    /// If start is provided, the list will start from the subaccount after the provided one.
    pub fn list_subaccounts(args: ListSubaccountsArgs) -> Vec<Subaccount> {
        Index::list_subaccounts(args.owner, args.start)
    }

    /// Get transactions for an account
    pub async fn get_account_transactions(
        args: GetAccountTransactionArgs,
    ) -> GetTransactionsResult {
        Index::get_account_transactions(args.account, args.start, args.max_results).await
    }

    /// Commit a transaction into the Index
    pub fn commit(id: u64, tx: Transaction) -> TxId {
        if !Inspect::inspect_is_archive_canister(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        Index::commit(id, tx)
    }
}

#[cfg(test)]
mod test {

    use did::ekoke_index::Transfer;
    use icrc::icrc1::account::Account;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_init_canister() {
        init_canister();
        assert_eq!(Configuration::get_archive_canister(), caller());
        assert_eq!(Configuration::get_ledger_canister(), caller());
    }

    #[test]
    fn test_should_get_ledger_id() {
        init_canister();
        assert_eq!(EkokeIndexCanister::ledger_id(), caller());
    }

    #[test]
    fn test_should_commit_tx() {
        init_canister();
        let tx = Transaction {
            kind: "transfer".to_string(),
            mint: None,
            burn: None,
            transfer: Some(Transfer {
                amount: 100_u64.into(),
                from: Account {
                    owner: Principal::management_canister(),
                    subaccount: Some([1u8; 32]),
                },
                to: Account {
                    owner: Principal::management_canister(),
                    subaccount: None,
                },
                spender: None,
                memo: None,
                created_at_time: None,
                fee: None,
            }),
            approve: None,
            timestamp: 0,
        };
        assert_eq!(EkokeIndexCanister::commit(0, tx.clone()), 0u64);
        assert_eq!(EkokeIndexCanister::commit(1, tx.clone()), 1u64);
    }

    fn init_canister() {
        let init_data = EkokeIndexInitData {
            archive_id: caller(),
            ledger_id: caller(),
        };
        EkokeIndexCanister::init(init_data);
    }
}
