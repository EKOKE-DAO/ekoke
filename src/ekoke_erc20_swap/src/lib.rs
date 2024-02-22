//! # Ekoke Ledger canister
//!
//! The ekoke canister serves a ICRC-2 token called $EKOKE, which is the reward token for Deferred transactions.
//! It is a deflationary token which ...

mod abi;
mod app;
mod constants;
mod inspect;
mod utils;

use candid::{candid_method, Nat, Principal};
use did::ekoke::{EkokeResult, PicoEkoke};
use did::ekoke_erc20_swap::EkokeErc20SwapInitData;
use did::H160;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{init, post_upgrade, query, update};

use self::app::EkokeErc20SwapCanister;

#[init]
pub fn init(data: EkokeErc20SwapInitData) {
    EkokeErc20SwapCanister::init(data);
}

#[post_upgrade]
pub fn post_upgrade() {
    EkokeErc20SwapCanister::post_upgrade();
}

#[update]
#[candid_method(update)]
pub async fn swap_fee() -> EkokeResult<u64> {
    EkokeErc20SwapCanister::swap_fee().await
}

#[update]
#[candid_method(update)]
pub async fn swap(
    recipient: H160,
    amount: PicoEkoke,
    from_subaccount: Option<[u8; 32]>,
) -> EkokeResult<String> {
    EkokeErc20SwapCanister::swap(recipient, amount, from_subaccount).await
}

#[query]
#[candid_method(query)]
pub fn admin_cycles() -> Nat {
    EkokeErc20SwapCanister::admin_cycles()
}

#[update]
#[candid_method(update)]
pub fn admin_set_cketh_ledger_canister(canister_id: Principal) {
    EkokeErc20SwapCanister::admin_set_cketh_ledger_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_cketh_minter_canister(canister_id: Principal) {
    EkokeErc20SwapCanister::admin_set_cketh_minter_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_erc20_bridge_address(address: H160) {
    EkokeErc20SwapCanister::admin_set_erc20_bridge_address(address)
}

#[query]
#[candid_method(query)]
pub async fn admin_eth_wallet_address() -> H160 {
    EkokeErc20SwapCanister::admin_eth_wallet_address().await
}

#[update]
#[candid_method(update)]
pub fn admin_set_erc20_gas_price(gas_price: u64) {
    EkokeErc20SwapCanister::admin_set_erc20_gas_price(gas_price)
}

// http transform
#[query]
#[candid_method(query)]
fn http_transform_send_tx(raw: TransformArgs) -> HttpResponse {
    raw.response
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
