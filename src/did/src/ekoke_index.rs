mod transaction;

use candid::{CandidType, Deserialize, Nat, Principal};
use icrc::icrc1::account::{Account, Subaccount};

pub use self::transaction::{Approve, Burn, Mint, Transaction, TransactionWithId, Transfer};

pub type TxId = Nat;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EkokeIndexInitData {
    /// ID of the archive canister
    pub archive_id: Principal,
    /// ID of ekoke-ledger canister
    pub ledger_id: Principal,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GetAccountTransactionArgs {
    pub account: Account,
    pub start: Option<TxId>,
    pub max_results: Nat,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GetTransactions {
    pub transactions: Vec<TransactionWithId>,
    pub oldest_tx_id: Option<TxId>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GetTransactionsErr {
    pub message: String,
}

pub type GetTransactionsResult = Result<GetTransactions, GetTransactionsErr>;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ListSubaccountsArgs {
    pub owner: Principal,
    pub start: Option<Subaccount>,
}
