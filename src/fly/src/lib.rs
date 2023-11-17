//! # Fly
//!
//! The fly canister serves a ICRC-2 token called $FLY, which is the reward token for Dilazionato transactions.
//! It is a deflationary token which ...

mod app;
mod constants;
mod inspect;
mod utils;

use app::FlyCanister;
use candid::{candid_method, Nat, Principal};
use did::fly::{FlyInitData, FlyResult, PicoFly, Role};
use did::ID;
use ic_cdk_macros::{init, post_upgrade, query, update};
use icrc::icrc1::account::Account;

#[init]
pub fn init(data: FlyInitData) {
    FlyCanister::init(data);
}

#[post_upgrade]
pub fn post_upgrade() {
    FlyCanister::post_upgrade();
}

#[update]
#[candid_method(update)]
pub fn get_contract_reward(contract_id: ID, installments: PicoFly) -> FlyResult<PicoFly> {
    FlyCanister::get_contract_reward(contract_id, installments)
}

#[update]
#[candid_method(update)]
pub fn reserve_pool(from: Account, contract_id: ID, picofly_amount: PicoFly) -> FlyResult<PicoFly> {
    FlyCanister::reserve_pool(from, contract_id, picofly_amount)
}

#[update]
#[candid_method(update)]
pub fn admin_set_role(principal: Principal, role: Role) {
    FlyCanister::admin_set_role(principal, role)
}

#[update]
#[candid_method(update)]
pub fn admin_remove_role(principal: Principal, role: Role) -> FlyResult<()> {
    FlyCanister::admin_remove_role(principal, role)
}

#[update]
#[candid_method(update)]
pub fn admin_cycles() -> Nat {
    FlyCanister::admin_cycles()
}

#[update]
#[candid_method(update)]
pub fn admin_burn(amount: PicoFly) -> FlyResult<()> {
    FlyCanister::admin_burn(amount)
}

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
