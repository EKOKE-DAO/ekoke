use candid::{candid_method, Nat, Principal};
use did::ID;
use ic_cdk_macros::{init, post_upgrade, query, update};

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
