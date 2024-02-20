//! # Ekoke Index canister
//!
//! The ekoke index provides a map of which transactions are relevant for a given account

mod app;
mod inspect;
mod utils;

use candid::{candid_method, Principal};
use did::ekoke_index::{
    EkokeIndexInitData, GetAccountTransactionArgs, GetTransactionsResult, ListSubaccountsArgs,
    Transaction, TxId,
};
use ic_cdk_macros::{init, post_upgrade, query, update};
use icrc::icrc1::account::Subaccount;

use self::app::EkokeIndexCanister;

#[init]
pub fn init(data: EkokeIndexInitData) {
    EkokeIndexCanister::init(data);
}

#[post_upgrade]
pub fn post_upgrade() {
    EkokeIndexCanister::post_upgrade();
}

#[query]
#[candid_method(query)]
pub fn ledger_id() -> Principal {
    EkokeIndexCanister::ledger_id()
}

#[query]
#[candid_method(query)]
pub fn list_subaccounts(args: ListSubaccountsArgs) -> Vec<Subaccount> {
    EkokeIndexCanister::list_subaccounts(args)
}

#[update]
#[candid_method(update)]
pub fn get_account_transactions(args: GetAccountTransactionArgs) -> GetTransactionsResult {
    EkokeIndexCanister::get_account_transactions(args)
}

#[update]
#[candid_method(update)]
pub fn commit(tx: Transaction) -> TxId {
    EkokeIndexCanister::commit(tx)
}

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
