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
use ic_cdk_macros::{init, query, update};
use icrc::icrc1::account::Subaccount;

use self::app::EkokeIndexCanister;

#[init]
pub fn init(data: EkokeIndexInitData) {
    EkokeIndexCanister::init(data);
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
pub async fn get_account_transactions(args: GetAccountTransactionArgs) -> GetTransactionsResult {
    EkokeIndexCanister::get_account_transactions(args).await
}

#[update]
#[candid_method(update)]
pub fn commit(id: u64, tx: Transaction) -> TxId {
    EkokeIndexCanister::commit(id, tx)
}

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}

/// GetRandom fixup to allow getrandom compilation.
/// A getrandom implementation that always fails
///
/// This is a workaround for the fact that the `getrandom` crate does not compile
/// for the `wasm32-unknown-ic` target. This is a dummy implementation that always
/// fails with `Error::UNSUPPORTED`.
pub fn getrandom_always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

getrandom::register_custom_getrandom!(getrandom_always_fail);
