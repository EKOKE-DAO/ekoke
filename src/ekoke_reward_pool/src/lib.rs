//! # Ekoke Ledger canister
//!
//! The ekoke canister serves a ICRC-2 token called $EKOKE, which is the reward token for Deferred transactions.
//! It is a deflationary token which ...

mod app;
mod constants;
mod inspect;
mod utils;

use candid::{candid_method, Nat, Principal};
use did::ekoke::{Ekoke, EkokeResult};
use did::ekoke_reward_pool::{EkokeRewardPoolInitData, Role};
use did::ID;
use ic_cdk_macros::{init, query, update};
use icrc::icrc1::account::Account;

use self::app::EkokeRewardPoolCanister;

#[init]
pub fn init(data: EkokeRewardPoolInitData) {
    EkokeRewardPoolCanister::init(data);
}

#[update]
#[candid_method(update)]
pub async fn get_contract_reward(contract_id: ID, installments: u64) -> EkokeResult<Ekoke> {
    EkokeRewardPoolCanister::get_contract_reward(contract_id, installments).await
}

#[update]
#[candid_method(update)]
pub async fn send_reward(contract_id: ID, amount: Ekoke, buyer: Account) -> EkokeResult<()> {
    EkokeRewardPoolCanister::send_reward(contract_id, amount, buyer).await
}

#[query]
#[candid_method(query)]
pub async fn available_liquidity() -> EkokeResult<Ekoke> {
    EkokeRewardPoolCanister::available_liquidity().await
}

#[update]
#[candid_method(update)]
pub async fn reserve_pool(
    contract_id: ID,
    amount_amount: Ekoke,
    from_subaccount: Option<[u8; 32]>,
) -> EkokeResult<Ekoke> {
    EkokeRewardPoolCanister::reserve_pool(contract_id, amount_amount, from_subaccount).await
}

#[update]
#[candid_method(update)]
pub fn admin_set_ledger_canister(ledger_id: Principal) {
    EkokeRewardPoolCanister::admin_set_ledger_canister(ledger_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_role(principal: Principal, role: Role) {
    EkokeRewardPoolCanister::admin_set_role(principal, role)
}

#[update]
#[candid_method(update)]
pub fn admin_remove_role(principal: Principal, role: Role) -> EkokeResult<()> {
    EkokeRewardPoolCanister::admin_remove_role(principal, role)
}

#[query]
#[candid_method(query)]
pub fn admin_cycles() -> Nat {
    EkokeRewardPoolCanister::admin_cycles()
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
