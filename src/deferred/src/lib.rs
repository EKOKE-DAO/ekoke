//! # Deferred
//!
//! Deferred is a canister serving a DIP721 NFT contract that allows to create
//! a financial tool to sell any kind of entity (e.g. a house, a car, a boat, etc.) or to get
//! financing from third parties buying the NFTs and getting rewards in $EKOKE tokens

use candid::{candid_method, Nat, Principal};
use did::deferred::{
    Agency, Contract, ContractRegistration, DeferredInitData, DeferredResult, Role, TokenInfo,
};
use did::{HttpRequest, HttpResponse, ID};
use dip721::{Dip721 as _, GenericValue, TokenIdentifier};
use ic_cdk_macros::{init, post_upgrade, query, update};

mod app;
mod client;
mod constants;
mod http;
mod inspect;
mod utils;

use app::Deferred;

#[init]
pub fn init(init_data: DeferredInitData) {
    Deferred::init(init_data);
}

#[post_upgrade]
pub fn post_upgrade() {
    Deferred::post_upgrade();
}

// MHSC api

#[update]
#[candid_method(update)]
pub fn register_contract(data: ContractRegistration) -> DeferredResult<Nat> {
    Deferred::register_contract(data)
}

#[update]
#[candid_method(update)]
pub async fn sign_contract(contract_id: ID) -> DeferredResult<()> {
    Deferred::sign_contract(contract_id).await
}

#[update]
#[candid_method(update)]
pub async fn increment_contract_value(
    contract_id: ID,
    value: u64,
    installments: u64,
) -> DeferredResult<()> {
    Deferred::increment_contract_value(contract_id, value, installments).await
}

#[query]
#[candid_method(query)]
pub fn get_token(token_id: TokenIdentifier) -> Option<TokenInfo> {
    Deferred::get_token(&token_id)
}

#[query]
#[candid_method(query)]
pub fn get_contract(id: ID) -> Option<Contract> {
    Deferred::get_contract(&id)
}

#[query]
#[candid_method(query)]
pub fn get_agencies() -> Vec<Agency> {
    Deferred::get_agencies()
}

#[update]
#[candid_method(update)]
pub fn remove_agency(wallet: Principal) -> DeferredResult<()> {
    Deferred::remove_agency(wallet)
}

#[query]
#[candid_method(query)]
pub fn get_signed_contracts() -> Vec<ID> {
    Deferred::get_signed_contracts()
}

#[update]
#[candid_method(update)]
pub fn update_contract_property(
    contract_id: ID,
    key: String,
    value: GenericValue,
) -> DeferredResult<()> {
    Deferred::update_contract_property(contract_id, key, value)
}

#[query]
#[candid_method(query)]
pub fn admin_get_unsigned_contracts() -> Vec<ID> {
    Deferred::admin_get_unsigned_contracts()
}

#[update]
#[candid_method(update)]
pub fn update_contract_buyers(contract_id: ID, buyers: Vec<Principal>) -> DeferredResult<()> {
    Deferred::update_contract_buyers(contract_id, buyers)
}

#[update]
#[candid_method(update)]
pub fn admin_set_ekoke_ledger_canister(canister_id: Principal) {
    Deferred::admin_set_ekoke_ledger_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_marketplace_canister(canister_id: Principal) {
    Deferred::admin_set_marketplace_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_role(principal: Principal, role: Role) {
    Deferred::admin_set_role(principal, role)
}

#[update]
#[candid_method(update)]
pub fn admin_remove_role(principal: Principal, role: Role) -> DeferredResult<()> {
    Deferred::admin_remove_role(principal, role)
}

#[update]
#[candid_method(update)]
pub fn admin_register_agency(wallet: Principal, agency: Agency) {
    Deferred::admin_register_agency(wallet, agency)
}

// DIP721

#[query]
#[candid_method(query)]
pub fn metadata() -> dip721::Metadata {
    Deferred::metadata()
}

#[query]
#[candid_method(query)]
pub fn stats() -> dip721::Stats {
    Deferred::stats()
}

#[query]
#[candid_method(query)]
pub fn logo() -> Option<String> {
    Deferred::logo()
}

#[update]
#[candid_method(update)]
pub fn set_logo(logo: String) {
    Deferred::set_logo(logo)
}

#[query]
#[candid_method(query)]
pub fn name() -> Option<String> {
    Deferred::name()
}

#[update]
#[candid_method(update)]
pub fn set_name(name: String) {
    Deferred::set_name(name)
}

#[query]
#[candid_method(query)]
pub fn symbol() -> Option<String> {
    Deferred::symbol()
}

#[update]
#[candid_method(update)]
pub fn set_symbol(symbol: String) {
    Deferred::set_symbol(symbol)
}

#[query]
#[candid_method(query)]
pub fn custodians() -> Vec<Principal> {
    Deferred::custodians()
}

#[update]
#[candid_method(update)]
pub fn set_custodians(custodians: Vec<Principal>) {
    Deferred::set_custodians(custodians)
}

#[query]
#[candid_method(query)]
pub fn cycles() -> Nat {
    Deferred::cycles()
}

#[query]
#[candid_method(query)]
pub fn total_unique_holders() -> Nat {
    Deferred::total_unique_holders()
}

#[query]
#[candid_method(query)]
pub fn token_metadata(
    token_identifier: dip721::TokenIdentifier,
) -> Result<dip721::TokenMetadata, dip721::NftError> {
    Deferred::token_metadata(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn balance_of(owner: Principal) -> Result<Nat, dip721::NftError> {
    Deferred::balance_of(owner)
}

#[query]
#[candid_method(query)]
pub fn owner_of(
    token_identifier: dip721::TokenIdentifier,
) -> Result<Option<Principal>, dip721::NftError> {
    Deferred::owner_of(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn owner_token_identifiers(
    owner: Principal,
) -> Result<Vec<dip721::TokenIdentifier>, dip721::NftError> {
    Deferred::owner_token_identifiers(owner)
}

#[query]
#[candid_method(query)]
pub fn owner_token_metadata(
    owner: Principal,
) -> Result<Vec<dip721::TokenMetadata>, dip721::NftError> {
    Deferred::owner_token_metadata(owner)
}

#[query]
#[candid_method(query)]
pub fn operator_of(
    token_identifier: dip721::TokenIdentifier,
) -> Result<Option<Principal>, dip721::NftError> {
    Deferred::operator_of(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn operator_token_identifiers(
    operator: Principal,
) -> Result<Vec<dip721::TokenIdentifier>, dip721::NftError> {
    Deferred::operator_token_identifiers(operator)
}

#[query]
#[candid_method(query)]
pub fn operator_token_metadata(
    operator: Principal,
) -> Result<Vec<dip721::TokenMetadata>, dip721::NftError> {
    Deferred::operator_token_metadata(operator)
}

#[query]
#[candid_method(query)]
pub fn supported_interfaces() -> Vec<dip721::SupportedInterface> {
    Deferred::supported_interfaces()
}

#[query]
#[candid_method(query)]
pub fn total_supply() -> Nat {
    Deferred::total_supply()
}

#[update]
#[candid_method(update)]
pub fn approve(
    spender: Principal,
    token_identifier: dip721::TokenIdentifier,
) -> Result<dip721::TokenIdentifier, dip721::NftError> {
    Deferred::approve(spender, token_identifier)
}

#[update]
#[candid_method(update)]
pub fn set_approval_for_all(
    operator: Principal,
    approved: bool,
) -> Result<dip721::TokenIdentifier, dip721::NftError> {
    Deferred::set_approval_for_all(operator, approved)
}

#[update]
#[candid_method(update)]
pub fn is_approved_for_all(
    owner: Principal,
    operator: Principal,
) -> Result<bool, dip721::NftError> {
    Deferred::is_approved_for_all(owner, operator)
}

#[update]
#[candid_method(update)]
pub async fn transfer(
    to: Principal,
    token_identifier: dip721::TokenIdentifier,
) -> Result<Nat, dip721::NftError> {
    Deferred::transfer(to, token_identifier).await
}

#[update]
#[candid_method(update)]
pub async fn transfer_from(
    from: Principal,
    to: Principal,
    token_identifier: dip721::TokenIdentifier,
) -> Result<Nat, dip721::NftError> {
    Deferred::transfer_from(from, to, token_identifier).await
}

#[update]
#[candid_method(update)]
pub fn mint(
    to: Principal,
    token_identifier: dip721::TokenIdentifier,
    properties: Vec<(String, dip721::GenericValue)>,
) -> Result<Nat, dip721::NftError> {
    Deferred::mint(to, token_identifier, properties)
}

#[update]
#[candid_method(update)]
pub fn burn(
    token_identifier: dip721::TokenIdentifier,
) -> Result<dip721::TokenIdentifier, dip721::NftError> {
    Deferred::burn(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn transaction(tx_id: Nat) -> Result<dip721::TxEvent, dip721::NftError> {
    Deferred::transaction(tx_id)
}

#[query]
#[candid_method(query)]
pub fn total_transactions() -> Nat {
    Deferred::total_transactions()
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
