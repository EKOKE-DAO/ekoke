//! # Fly
//!
//! The fly canister serves a ICRC-2 token called $FLY, which is the reward token for Dilazionato transactions.
//! It is a deflationary token which ...

mod app;

use candid::{candid_method, Nat, Principal};
use did::fly::FlyResult;
use did::ID;
use ic_cdk_macros::{init, post_upgrade, query, update};

use app::FlyCanister;

#[init]
pub fn init() {
    FlyCanister::init();
}

#[post_upgrade]
pub fn post_upgrade() {
    FlyCanister::post_upgrade();
}

#[update]
#[candid_method(update)]
pub fn reserve_pool(contract_id: ID, mfly_amount: u64) -> FlyResult<u64> {
    FlyCanister::reserve_pool(contract_id, mfly_amount)
}

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
