//! # Ekoke Ledger canister
//!
//! The ekoke canister serves a ICRC-2 token called $EKOKE, which is the reward token for Deferred transactions.
//! It is a deflationary token which ...

mod abi;
mod app;
mod constants;
mod http;
mod inspect;
mod utils;

use candid::{candid_method, Nat, Principal};
use did::ekoke::{
    EkokeInitData, EkokeResult, LiquidityPoolAccounts, LiquidityPoolBalance, PicoEkoke, Role,
};
use did::{H160, ID};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{init, post_upgrade, query, update};
use icrc::icrc::generic_metadata_value::MetadataValue;
use icrc::icrc1::account::Account;
use icrc::icrc1::transfer::TransferArg;
use icrc::icrc1::{self, transfer as icrc1_transfer, Icrc1 as _};
use icrc::icrc2::{self, Icrc2 as _};

use self::app::EkokeCanister;

#[init]
pub fn init(data: EkokeInitData) {
    EkokeCanister::init(data);
}

#[post_upgrade]
pub fn post_upgrade() {
    EkokeCanister::post_upgrade();
}

#[update]
#[candid_method(update)]
pub async fn get_contract_reward(contract_id: ID, installments: u64) -> EkokeResult<PicoEkoke> {
    EkokeCanister::get_contract_reward(contract_id, installments).await
}

#[update]
#[candid_method(update)]
pub async fn send_reward(contract_id: ID, picoekoke: PicoEkoke, buyer: Account) -> EkokeResult<()> {
    EkokeCanister::send_reward(contract_id, picoekoke, buyer).await
}

#[update]
#[candid_method(update)]
pub async fn reserve_pool(
    contract_id: ID,
    picoekoke_amount: PicoEkoke,
    from_subaccount: Option<[u8; 32]>,
) -> EkokeResult<PicoEkoke> {
    EkokeCanister::reserve_pool(contract_id, picoekoke_amount, from_subaccount).await
}

#[query]
#[candid_method(query)]
pub async fn liquidity_pool_balance() -> EkokeResult<LiquidityPoolBalance> {
    EkokeCanister::liquidity_pool_balance().await
}

#[query]
#[candid_method(query)]
pub fn liquidity_pool_accounts() -> LiquidityPoolAccounts {
    EkokeCanister::liquidity_pool_accounts()
}

#[update]
#[candid_method(update)]
pub async fn erc20_swap_fee() -> EkokeResult<u64> {
    EkokeCanister::erc20_swap_fee().await
}

#[update]
#[candid_method(update)]
pub async fn erc20_swap(
    recipient: H160,
    amount: PicoEkoke,
    from_subaccount: Option<[u8; 32]>,
) -> EkokeResult<String> {
    EkokeCanister::erc20_swap(recipient, amount, from_subaccount).await
}

#[update]
#[candid_method(update)]
pub fn admin_set_role(principal: Principal, role: Role) {
    EkokeCanister::admin_set_role(principal, role)
}

#[update]
#[candid_method(update)]
pub fn admin_remove_role(principal: Principal, role: Role) -> EkokeResult<()> {
    EkokeCanister::admin_remove_role(principal, role)
}

#[query]
#[candid_method(query)]
pub fn admin_cycles() -> Nat {
    EkokeCanister::admin_cycles()
}

#[update]
#[candid_method(update)]
pub fn admin_burn(amount: PicoEkoke) -> EkokeResult<()> {
    EkokeCanister::admin_burn(amount)
}

#[update]
#[candid_method(update)]
pub fn admin_set_swap_account(account: Account) {
    EkokeCanister::admin_set_swap_account(account)
}

#[update]
#[candid_method(update)]
pub fn admin_set_xrc_canister(canister_id: Principal) {
    EkokeCanister::admin_set_xrc_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_ckbtc_canister(canister_id: Principal) {
    EkokeCanister::admin_set_ckbtc_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_icp_ledger_canister(canister_id: Principal) {
    EkokeCanister::admin_set_icp_ledger_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_cketh_ledger_canister(canister_id: Principal) {
    EkokeCanister::admin_set_cketh_ledger_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_cketh_minter_canister(canister_id: Principal) {
    EkokeCanister::admin_set_cketh_minter_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_erc20_bridge_address(address: H160) {
    EkokeCanister::admin_set_erc20_bridge_address(address)
}

#[update]
#[candid_method(update)]
pub fn admin_set_erc20_gas_price(gas_price: u64) {
    EkokeCanister::admin_set_erc20_gas_price(gas_price)
}

#[query]
#[candid_method(query)]
pub async fn admin_eth_wallet_address() -> H160 {
    EkokeCanister::admin_eth_wallet_address().await
}

// icrc-1

#[query]
#[candid_method(query)]
pub fn icrc1_name() -> &'static str {
    EkokeCanister::icrc1_name()
}

#[query]
#[candid_method(query)]
pub fn icrc1_symbol() -> &'static str {
    EkokeCanister::icrc1_symbol()
}

#[query]
#[candid_method(query)]
pub fn icrc1_decimals() -> u8 {
    EkokeCanister::icrc1_decimals()
}

#[query]
#[candid_method(query)]
pub fn icrc1_fee() -> Nat {
    EkokeCanister::icrc1_fee()
}

#[query]
#[candid_method(query)]
pub fn icrc1_metadata() -> Vec<(String, MetadataValue)> {
    EkokeCanister::icrc1_metadata()
}

#[query]
#[candid_method(query)]
pub fn icrc1_total_supply() -> Nat {
    EkokeCanister::icrc1_total_supply()
}

#[query]
#[candid_method(query)]
pub fn icrc1_balance_of(account: Account) -> Nat {
    EkokeCanister::icrc1_balance_of(account)
}

#[update]
#[candid_method(update)]
pub async fn icrc1_transfer(
    transfer_args: TransferArg,
) -> Result<Nat, icrc1_transfer::TransferError> {
    EkokeCanister::icrc1_transfer(transfer_args).await
}

#[query]
#[candid_method(query)]
pub fn icrc1_supported_standards() -> Vec<icrc1::TokenExtension> {
    EkokeCanister::icrc1_supported_standards()
}

#[update]
#[candid_method(update)]
pub async fn icrc2_approve(
    args: icrc2::approve::ApproveArgs,
) -> Result<Nat, icrc2::approve::ApproveError> {
    EkokeCanister::icrc2_approve(args).await
}

#[update]
#[candid_method(update)]
pub async fn icrc2_transfer_from(
    args: icrc2::transfer_from::TransferFromArgs,
) -> Result<Nat, icrc2::transfer_from::TransferFromError> {
    EkokeCanister::icrc2_transfer_from(args).await
}

#[query]
#[candid_method(query)]
pub fn icrc2_allowance(args: icrc2::allowance::AllowanceArgs) -> icrc2::allowance::Allowance {
    EkokeCanister::icrc2_allowance(args)
}

// http transform
#[query]
#[candid_method(query)]
fn http_transform_send_tx(raw: TransformArgs) -> HttpResponse {
    raw.response
}

// HTTP endpoint
#[query]
#[candid_method(query)]
pub async fn http_request(req: did::HttpRequest) -> did::HttpResponse {
    http::HttpApi::handle_http_request(req).await
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
