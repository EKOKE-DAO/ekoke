//! # Fly
//!
//! The fly canister serves a ICRC-2 token called $FLY, which is the reward token for Dilazionato transactions.
//! It is a deflationary token which ...

mod app;

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
