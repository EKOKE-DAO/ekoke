mod app;
mod client;
mod constants;
mod inspect;
mod utils;

use candid::{candid_method, Nat, Principal};
use did::marketplace::{MarketplaceInitData, MarketplaceResult};
use dip721::TokenIdentifier;
use ic_cdk_macros::{init, query, update};

use self::app::Marketplace;

#[init]
pub fn init(data: MarketplaceInitData) {
    Marketplace::init(data);
}

#[query]
#[candid_method(query)]
pub fn admin_cycles() -> Nat {
    Marketplace::admin_cycles()
}

#[update]
#[candid_method(update)]
pub fn admin_set_admins(admins: Vec<Principal>) -> MarketplaceResult<()> {
    Marketplace::admin_set_admins(admins)
}

#[update]
#[candid_method(update)]
pub fn admin_set_deferred_canister(canister: Principal) {
    Marketplace::admin_set_deferred_canister(canister)
}

#[update]
#[candid_method(update)]
pub fn admin_set_fly_canister(canister: Principal) {
    Marketplace::admin_set_fly_canister(canister)
}

#[update]
#[candid_method(update)]
pub fn admin_set_interest_rate_for_buyer(interest_rate: f64) {
    Marketplace::admin_set_interest_rate_for_buyer(interest_rate)
}

#[update]
#[candid_method(update)]
pub async fn get_token_price_icp(token_id: TokenIdentifier) -> MarketplaceResult<u64> {
    Marketplace::get_token_price_icp(token_id).await
}

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
