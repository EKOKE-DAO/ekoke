use candid::{candid_method, Nat, Principal};
use did::dilazionato::{Contract, ContractRegistration, SellContractInitData, SellContractResult};
use did::ID;
use dip721::Dip721 as _;
use ic_cdk_macros::{init, post_upgrade, query, update};

mod app;
mod client;
mod constants;
mod inspect;
mod utils;

use app::SellContract;

#[init]
pub fn init(init_data: SellContractInitData) {
    SellContract::init(init_data);
}

#[post_upgrade]
pub fn post_upgrade() {
    SellContract::post_upgrade();
}

// MHSC api

#[update]
#[candid_method(update)]
pub async fn register_contract(data: ContractRegistration) -> SellContractResult<()> {
    SellContract::register_contract(data).await
}

#[update]
#[candid_method(update)]
pub async fn seller_increment_contract_value(
    contract_id: ID,
    value: u64,
    installments: u64,
) -> SellContractResult<()> {
    SellContract::seller_increment_contract_value(contract_id, value, installments).await
}

#[query]
#[candid_method(query)]
pub fn get_contract(id: ID) -> Option<Contract> {
    SellContract::get_contract(&id)
}

#[query]
#[candid_method(query)]
pub fn get_contracts() -> Vec<ID> {
    SellContract::get_contracts()
}

#[update]
#[candid_method(update)]
pub fn update_contract_buyers(contract_id: ID, buyers: Vec<Principal>) -> SellContractResult<()> {
    SellContract::update_contract_buyers(contract_id, buyers)
}

#[update]
#[candid_method(update)]
pub fn admin_set_fly_canister(canister_id: Principal) {
    SellContract::admin_set_fly_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_marketplace_canister(canister_id: Principal) {
    SellContract::admin_set_marketplace_canister(canister_id)
}

// DIP721

#[query]
#[candid_method(query)]
pub fn metadata() -> dip721::Metadata {
    SellContract::metadata()
}

#[query]
#[candid_method(query)]
pub fn stats() -> dip721::Stats {
    SellContract::stats()
}

#[query]
#[candid_method(query)]
pub fn logo() -> Option<String> {
    SellContract::logo()
}

#[update]
#[candid_method(update)]
pub fn set_logo(logo: String) {
    SellContract::set_logo(logo)
}

#[query]
#[candid_method(query)]
pub fn name() -> Option<String> {
    SellContract::name()
}

#[update]
#[candid_method(update)]
pub fn set_name(name: String) {
    SellContract::set_name(name)
}

#[query]
#[candid_method(query)]
pub fn symbol() -> Option<String> {
    SellContract::symbol()
}

#[update]
#[candid_method(update)]
pub fn set_symbol(symbol: String) {
    SellContract::set_symbol(symbol)
}

#[query]
#[candid_method(query)]
pub fn custodians() -> Vec<Principal> {
    SellContract::custodians()
}

#[update]
#[candid_method(update)]
pub fn set_custodians(custodians: Vec<Principal>) {
    SellContract::set_custodians(custodians)
}

#[query]
#[candid_method(query)]
pub fn cycles() -> Nat {
    SellContract::cycles()
}

#[query]
#[candid_method(query)]
pub fn total_unique_holders() -> Nat {
    SellContract::total_unique_holders()
}

#[query]
#[candid_method(query)]
pub fn token_metadata(
    token_identifier: dip721::TokenIdentifier,
) -> Result<dip721::TokenMetadata, dip721::NftError> {
    SellContract::token_metadata(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn balance_of(owner: Principal) -> Result<Nat, dip721::NftError> {
    SellContract::balance_of(owner)
}

#[query]
#[candid_method(query)]
pub fn owner_of(
    token_identifier: dip721::TokenIdentifier,
) -> Result<Option<Principal>, dip721::NftError> {
    SellContract::owner_of(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn owner_token_identifiers(
    owner: Principal,
) -> Result<Vec<dip721::TokenIdentifier>, dip721::NftError> {
    SellContract::owner_token_identifiers(owner)
}

#[query]
#[candid_method(query)]
pub fn owner_token_metadata(
    owner: Principal,
) -> Result<Vec<dip721::TokenMetadata>, dip721::NftError> {
    SellContract::owner_token_metadata(owner)
}

#[query]
#[candid_method(query)]
pub fn operator_of(
    token_identifier: dip721::TokenIdentifier,
) -> Result<Option<Principal>, dip721::NftError> {
    SellContract::operator_of(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn operator_token_identifiers(
    operator: Principal,
) -> Result<Vec<dip721::TokenIdentifier>, dip721::NftError> {
    SellContract::operator_token_identifiers(operator)
}

#[query]
#[candid_method(query)]
pub fn operator_token_metadata(
    operator: Principal,
) -> Result<Vec<dip721::TokenMetadata>, dip721::NftError> {
    SellContract::operator_token_metadata(operator)
}

#[query]
#[candid_method(query)]
pub fn supported_interfaces() -> Vec<dip721::SupportedInterface> {
    SellContract::supported_interfaces()
}

#[query]
#[candid_method(query)]
pub fn total_supply() -> Nat {
    SellContract::total_supply()
}

#[update]
#[candid_method(update)]
pub fn approve(
    spender: Principal,
    token_identifier: dip721::TokenIdentifier,
) -> Result<dip721::TokenIdentifier, dip721::NftError> {
    SellContract::approve(spender, token_identifier)
}

#[update]
#[candid_method(update)]
pub fn set_approval_for_all(
    operator: Principal,
    approved: bool,
) -> Result<dip721::TokenIdentifier, dip721::NftError> {
    SellContract::set_approval_for_all(operator, approved)
}

#[update]
#[candid_method(update)]
pub fn is_approved_for_all(
    owner: Principal,
    operator: Principal,
) -> Result<bool, dip721::NftError> {
    SellContract::is_approved_for_all(owner, operator)
}

#[update]
#[candid_method(update)]
pub async fn transfer(
    to: Principal,
    token_identifier: dip721::TokenIdentifier,
) -> Result<Nat, dip721::NftError> {
    SellContract::transfer(to, token_identifier).await
}

#[update]
#[candid_method(update)]
pub async fn transfer_from(
    from: Principal,
    to: Principal,
    token_identifier: dip721::TokenIdentifier,
) -> Result<Nat, dip721::NftError> {
    SellContract::transfer_from(from, to, token_identifier).await
}

#[update]
#[candid_method(update)]
pub fn mint(
    to: Principal,
    token_identifier: dip721::TokenIdentifier,
    properties: Vec<(String, dip721::GenericValue)>,
) -> Result<Nat, dip721::NftError> {
    SellContract::mint(to, token_identifier, properties)
}

#[update]
#[candid_method(update)]
pub fn burn(
    token_identifier: dip721::TokenIdentifier,
) -> Result<dip721::TokenIdentifier, dip721::NftError> {
    SellContract::burn(token_identifier)
}

#[query]
#[candid_method(query)]
pub fn transaction(tx_id: Nat) -> Result<dip721::TxEvent, dip721::NftError> {
    SellContract::transaction(tx_id)
}

#[query]
#[candid_method(query)]
pub fn total_transactions() -> Nat {
    SellContract::total_transactions()
}

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
