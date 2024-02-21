mod archive;
mod blocks;
mod configuration;
mod index_client;
mod inspect;
mod memory;
#[cfg(test)]
mod test_utils;

use did::ekoke_archive::{
    EkokeArchiveInitData, GetBlocksArg, GetBlocksRet, GetTransactionsArg, GetTransactionsRet,
    Transaction,
};
use num_traits::ToPrimitive;
use serde_bytes::ByteBuf;

use self::archive::Archive;
use self::blocks::Blocks;
use self::configuration::Configuration;
use self::index_client::IndexCanister;
pub use self::inspect::Inspect;
use crate::utils::caller;

pub struct EkokeArchiveCanister;

impl EkokeArchiveCanister {
    pub fn init(args: EkokeArchiveInitData) {
        Configuration::set_index_canister(args.index_id);
        Configuration::set_ledger_canister(args.ledger_id);
    }

    /// Append blocks
    pub fn append_blocks(new_blocks: Vec<ByteBuf>) {
        if !Inspect::inspect_is_ledger_canister(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        if let Err(err) = Blocks::append_blocks(new_blocks) {
            ic_cdk::trap(&err);
        }
    }

    pub fn get_blocks(_: GetBlocksArg) -> GetBlocksRet {
        GetBlocksRet { blocks: vec![] }
    }

    pub fn remaining_capacity() -> u64 {
        Blocks::remaining_capacity()
    }

    pub fn get_transaction(id: u64) -> Option<Transaction> {
        Archive::get_transaction(id)
    }

    pub fn get_transactions(args: GetTransactionsArg) -> GetTransactionsRet {
        let transactions = Archive::get_transactions(
            args.start.0.to_u64().unwrap_or_default(),
            args.length.0.to_u64().unwrap_or_default(),
        );

        GetTransactionsRet { transactions }
    }

    /// Commit a transaction into the Index
    pub async fn commit(tx: Transaction) -> u64 {
        if !Inspect::inspect_is_ledger_canister(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        let id = Archive::commit(tx.clone());
        // forward transaction to index
        IndexCanister::commit(id, tx).await;

        id
    }
}

#[cfg(test)]
mod test {

    use candid::Principal;
    use did::ekoke_index::Transfer;
    use icrc::icrc1::account::Account;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_init_canister() {
        init_canister();
        assert_eq!(Configuration::get_ledger_canister(), caller());
        assert_eq!(Configuration::get_index_canister(), caller());
    }

    #[tokio::test]
    async fn test_should_commit_tx() {
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
        assert_eq!(EkokeArchiveCanister::commit(tx.clone()).await, 0);
        assert_eq!(EkokeArchiveCanister::commit(tx.clone()).await, 1);
    }

    fn init_canister() {
        let init_data = EkokeArchiveInitData {
            index_id: caller(),
            ledger_id: caller(),
        };
        EkokeArchiveCanister::init(init_data);
    }
}
