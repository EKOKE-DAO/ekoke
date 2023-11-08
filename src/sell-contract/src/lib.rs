use ic_cdk_macros::{init, post_upgrade};

mod app;
mod constants;
mod inspect;
mod utils;

use app::SellContract;

#[init]
pub fn init() {
    SellContract::init();
}

#[post_upgrade]
pub fn post_upgrade() {
    SellContract::post_upgrade();
}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
