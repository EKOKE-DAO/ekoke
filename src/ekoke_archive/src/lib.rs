//! # Ekoke Archive canister
//!
//! The ekoke archive canister provides a history of the transactions that have been processed by the ledger

mod app;
mod inspect;
mod utils;

use candid::candid_method;
use did::ekoke_archive::{
    EkokeArchiveInitData, GetBlocksArg, GetBlocksRet, GetTransactionsArg, GetTransactionsRet,
    Transaction,
};
use ic_cdk_macros::{init, post_upgrade, query, update};
use serde_bytes::ByteBuf;

use self::app::EkokeArchiveCanister;

#[init]
pub fn init(args: EkokeArchiveInitData) {
    EkokeArchiveCanister::init(args)
}

#[post_upgrade]
pub fn post_upgrade() {}

#[update]
#[candid_method(update)]
pub fn append_blocks(blocks: Vec<ByteBuf>) {
    EkokeArchiveCanister::append_blocks(blocks)
}

#[query]
#[candid_method(query)]
pub fn get_blocks(args: GetBlocksArg) -> GetBlocksRet {
    EkokeArchiveCanister::get_blocks(args)
}

#[query]
#[candid_method(query)]
pub fn remaining_capacity() -> u64 {
    EkokeArchiveCanister::remaining_capacity()
}

#[query]
#[candid_method(query)]
pub fn get_transaction(tx_id: u64) -> Option<Transaction> {
    EkokeArchiveCanister::get_transaction(tx_id)
}

#[query]
#[candid_method(query)]
pub fn get_transactions(args: GetTransactionsArg) -> GetTransactionsRet {
    EkokeArchiveCanister::get_transactions(args)
}

#[update]
#[candid_method(update)]
pub async fn commit(tx: Transaction) -> u64 {
    EkokeArchiveCanister::commit(tx).await
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
