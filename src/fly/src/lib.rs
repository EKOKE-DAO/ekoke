//! # Fly
//!
//! The fly canister serves a ICRC-2 token called $FLY, which is the reward token for Deferred transactions.
//! It is a deflationary token which ...

mod abi;
mod app;
mod constants;
mod inspect;
mod utils;

use candid::{candid_method, Nat, Principal};
use did::fly::{
    FlyInitData, FlyResult, LiquidityPoolAccounts, LiquidityPoolBalance, PicoFly, Role, Transaction,
};
use did::{H160, ID};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{init, post_upgrade, query, update};
use icrc::icrc::generic_metadata_value::MetadataValue;
use icrc::icrc1::account::Account;
use icrc::icrc1::transfer::TransferArg;
use icrc::icrc1::{self, transfer as icrc1_transfer, Icrc1 as _};
use icrc::icrc2::{self, Icrc2 as _};

use self::app::FlyCanister;

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
pub async fn get_contract_reward(contract_id: ID, installments: u64) -> FlyResult<PicoFly> {
    FlyCanister::get_contract_reward(contract_id, installments).await
}

#[update]
#[candid_method(update)]
pub async fn send_reward(contract_id: ID, picofly: PicoFly, buyer: Account) -> FlyResult<()> {
    FlyCanister::send_reward(contract_id, picofly, buyer).await
}

#[update]
#[candid_method(update)]
pub async fn reserve_pool(
    from: Account,
    contract_id: ID,
    picofly_amount: PicoFly,
) -> FlyResult<PicoFly> {
    FlyCanister::reserve_pool(from, contract_id, picofly_amount).await
}

#[query]
#[candid_method(query)]
pub async fn liquidity_pool_balance() -> FlyResult<LiquidityPoolBalance> {
    FlyCanister::liquidity_pool_balance().await
}

#[query]
#[candid_method(query)]
pub fn liquidity_pool_accounts() -> LiquidityPoolAccounts {
    FlyCanister::liquidity_pool_accounts()
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

#[query]
#[candid_method(query)]
pub fn admin_cycles() -> Nat {
    FlyCanister::admin_cycles()
}

#[update]
#[candid_method(update)]
pub fn admin_burn(amount: PicoFly) -> FlyResult<()> {
    FlyCanister::admin_burn(amount)
}

#[update]
#[candid_method(update)]
pub fn admin_set_swap_account(account: Account) {
    FlyCanister::admin_set_swap_account(account)
}

#[update]
#[candid_method(update)]
pub fn admin_set_xrc_canister(canister_id: Principal) {
    FlyCanister::admin_set_xrc_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_ckbtc_canister(canister_id: Principal) {
    FlyCanister::admin_set_ckbtc_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_icp_ledger_canister(canister_id: Principal) {
    FlyCanister::admin_set_icp_ledger_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_cketh_ledger_canister(canister_id: Principal) {
    FlyCanister::admin_set_cketh_ledger_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_cketh_minter_canister(canister_id: Principal) {
    FlyCanister::admin_set_cketh_minter_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_erc20_bridge_address(address: H160) {
    FlyCanister::admin_set_erc20_bridge_address(address)
}

#[update]
#[candid_method(update)]
pub fn admin_set_erc20_swap_fee(fee: u64) {
    FlyCanister::admin_set_erc20_swap_fee(fee)
}

#[query]
#[candid_method(query)]
pub fn get_transaction(id: u64) -> FlyResult<Transaction> {
    FlyCanister::get_transaction(id)
}

// icrc-1

#[query]
#[candid_method(query)]
pub fn icrc1_name() -> &'static str {
    FlyCanister::icrc1_name()
}

#[query]
#[candid_method(query)]
pub fn icrc1_symbol() -> &'static str {
    FlyCanister::icrc1_symbol()
}

#[query]
#[candid_method(query)]
pub fn icrc1_decimals() -> u8 {
    FlyCanister::icrc1_decimals()
}

#[query]
#[candid_method(query)]
pub fn icrc1_fee() -> Nat {
    FlyCanister::icrc1_fee()
}

#[query]
#[candid_method(query)]
pub fn icrc1_metadata() -> Vec<(String, MetadataValue)> {
    FlyCanister::icrc1_metadata()
}

#[query]
#[candid_method(query)]
pub fn icrc1_total_supply() -> Nat {
    FlyCanister::icrc1_total_supply()
}

#[query]
#[candid_method(query)]
pub fn icrc1_balance_of(account: Account) -> Nat {
    FlyCanister::icrc1_balance_of(account)
}

#[update]
#[candid_method(update)]
pub fn icrc1_transfer(transfer_args: TransferArg) -> Result<Nat, icrc1_transfer::TransferError> {
    FlyCanister::icrc1_transfer(transfer_args)
}

#[query]
#[candid_method(query)]
pub fn icrc1_supported_standards() -> Vec<icrc1::TokenExtension> {
    FlyCanister::icrc1_supported_standards()
}

#[update]
#[candid_method(update)]
pub fn icrc2_approve(
    args: icrc2::approve::ApproveArgs,
) -> Result<Nat, icrc2::approve::ApproveError> {
    FlyCanister::icrc2_approve(args)
}

#[update]
#[candid_method(update)]
pub fn icrc2_transfer_from(
    args: icrc2::transfer_from::TransferFromArgs,
) -> Result<Nat, icrc2::transfer_from::TransferFromError> {
    FlyCanister::icrc2_transfer_from(args)
}

#[query]
#[candid_method(query)]
pub fn icrc2_allowance(args: icrc2::allowance::AllowanceArgs) -> icrc2::allowance::Allowance {
    FlyCanister::icrc2_allowance(args)
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
