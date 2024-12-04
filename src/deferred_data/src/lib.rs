use candid::{candid_method, Nat, Principal};
use did::deferred::{
    Contract, ContractDocument, ContractDocumentData, DeferredDataInitData, DeferredDataResult,
    GenericValue, RestrictedProperty,
};
use did::{HttpRequest, HttpResponse, ID};
use ic_cdk::post_upgrade;
use ic_cdk_macros::{init, query, update};

mod app;
mod http;
mod inspect;
mod utils;

use app::DeferredData;
use ic_log::did::Pagination;
use ic_log::writer::Logs;

#[init]
#[candid_method(init)]
pub fn init(data: DeferredDataInitData) {
    DeferredData::init(data);
}

#[post_upgrade]
pub fn post_upgrade() {
    DeferredData::post_upgrade();
}

#[update]
#[candid_method(update)]
pub fn admin_set_minter(minter: Principal) -> DeferredDataResult<()> {
    DeferredData::admin_set_minter(minter)
}

#[query]
#[candid_method(query)]
pub fn admin_ic_logs(pagination: Pagination) -> Logs {
    DeferredData::admin_ic_logs(pagination)
}

#[query]
#[candid_method(query)]
pub fn admin_cycles() -> Nat {
    DeferredData::admin_cycles()
}

#[update]
#[candid_method(update)]
pub fn minter_create_contract(data: Contract) -> DeferredDataResult<()> {
    DeferredData::create_contract(data)
}

#[update]
#[candid_method(update)]
pub async fn minter_close_contract(contract_id: ID) -> DeferredDataResult<()> {
    DeferredData::close_contract(contract_id)
}

#[query]
#[candid_method(query)]
pub fn get_contract(id: ID) -> Option<Contract> {
    DeferredData::get_contract(&id, None)
}

#[query]
#[candid_method(query)]
pub fn get_contracts() -> Vec<ID> {
    DeferredData::get_contracts()
}

#[query]
#[candid_method(query)]
pub fn get_contract_document(
    contract_id: ID,
    document_id: ID,
) -> DeferredDataResult<ContractDocumentData> {
    DeferredData::get_contract_document(contract_id, document_id, None)
}

#[update]
#[candid_method(update)]
pub fn upload_contract_document(
    contract_id: ID,
    document: ContractDocument,
    data: Vec<u8>,
) -> DeferredDataResult<ID> {
    DeferredData::upload_contract_document(contract_id, document, data)
}

#[update]
#[candid_method(update)]
pub fn update_contract_property(
    contract_id: ID,
    key: String,
    value: GenericValue,
) -> DeferredDataResult<()> {
    DeferredData::update_contract_property(contract_id, key, value)
}

#[update]
#[candid_method(update)]
pub fn update_restricted_contract_property(
    contract_id: ID,
    key: String,
    value: RestrictedProperty,
) -> DeferredDataResult<()> {
    DeferredData::update_restricted_contract_property(contract_id, key, value)
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
