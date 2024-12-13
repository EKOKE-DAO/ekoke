//! # Deferred
//!
//! Deferred is a canister serving a DIP721 NFT contract that allows to create
//! a financial tool to sell any kind of entity (e.g. a house, a car, a boat, etc.) or to get
//! financing from third parties buying the NFTs and getting rewards in $EKOKE tokens

use candid::{candid_method, Nat, Principal};
use did::deferred::{
    Agency, ContractRegistration, DeferredMinterInitData, DeferredMinterResult, Role,
};
use did::{HttpRequest, HttpResponse, ID};
use ic_cdk::post_upgrade;
use ic_cdk_macros::{init, query, update};

mod app;
mod http;
mod inspect;
mod utils;

use app::DeferredMinter;
use ic_log::did::Pagination;
use ic_log::writer::Logs;

#[init]
pub fn init(init_data: DeferredMinterInitData) {
    DeferredMinter::init(init_data);
}

#[post_upgrade]
pub fn post_upgrade() {
    DeferredMinter::post_upgrade();
}

#[update]
#[candid_method(update)]
pub async fn get_eth_address() -> DeferredMinterResult<String> {
    DeferredMinter::get_eth_address().await
}

#[update]
#[candid_method(update)]
pub async fn create_contract(data: ContractRegistration) -> DeferredMinterResult<ID> {
    DeferredMinter::create_contract(data).await
}

#[update]
#[candid_method(update)]
pub async fn close_contract(contract_id: ID) -> DeferredMinterResult<()> {
    DeferredMinter::close_contract(contract_id).await
}

#[query]
#[candid_method(query)]
pub fn get_agencies() -> Vec<Agency> {
    DeferredMinter::get_agencies()
}

#[query]
#[candid_method(query)]
pub fn get_agency(id: Principal) -> Option<Agency> {
    DeferredMinter::get_agent(id)
}

#[update]
#[candid_method(update)]
pub fn remove_agency(wallet: Principal) -> DeferredMinterResult<()> {
    DeferredMinter::remove_agency(wallet)
}

#[update]
#[candid_method(update)]
pub fn admin_set_role(principal: Principal, role: Role) {
    DeferredMinter::admin_set_role(principal, role)
}

#[update]
#[candid_method(update)]
pub fn admin_remove_role(principal: Principal, role: Role) -> DeferredMinterResult<()> {
    DeferredMinter::admin_remove_role(principal, role)
}

#[update]
#[candid_method(update)]
pub fn admin_set_allowed_currencies(currencies: Vec<String>) {
    DeferredMinter::admin_set_allowed_currencies(currencies)
}

#[update]
#[candid_method(update)]
pub fn admin_set_custodians(custodians: Vec<Principal>) -> DeferredMinterResult<()> {
    DeferredMinter::admin_set_custodians(custodians)
}

#[query]
#[candid_method(query)]
pub fn admin_cycles() -> Nat {
    DeferredMinter::admin_cycles()
}

#[query]
#[candid_method(query)]
pub fn admin_ic_logs(pagination: Pagination) -> Logs {
    DeferredMinter::admin_ic_logs(pagination)
}

#[update]
#[candid_method(update)]
pub fn gas_station_set_gas_price(gas_price: u64) -> DeferredMinterResult<()> {
    DeferredMinter::gas_station_set_gas_price(gas_price)
}

#[update]
#[candid_method(update)]
pub fn admin_register_agency(wallet: Principal, agency: Agency) {
    DeferredMinter::admin_register_agency(wallet, agency)
}

// HTTP endpoint
#[query]
#[candid_method(query)]
pub async fn http_request(req: HttpRequest) -> HttpResponse {
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
