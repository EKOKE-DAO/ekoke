use candid::{candid_method, Principal};
use did::{
    sell_contract::{BuildingData, Contract, SellContractInitData, SellContractResult},
    ID,
};
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
pub async fn admin_register_contract(
    id: ID,
    seller: Principal,
    buyers: Vec<Principal>,
    expiration: String,
    value: u64,
    installments: u64,
    building_data: BuildingData,
) -> SellContractResult<()> {
    SellContract::admin_register_contract(
        id,
        seller,
        buyers,
        expiration,
        value,
        installments,
        building_data,
    )
    .await
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
pub fn admin_set_fly_canister(canister_id: Principal) {
    SellContract::admin_set_fly_canister(canister_id)
}

#[update]
#[candid_method(update)]
pub fn admin_set_marketplace_canister(canister_id: Principal) {
    SellContract::admin_set_marketplace_canister(canister_id)
}

// DIP721

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
