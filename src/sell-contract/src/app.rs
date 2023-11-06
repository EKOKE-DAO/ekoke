//! # Sell Contract
//!
//! API for sell contract

mod configuration;
mod memory;
mod storage;

use candid::Principal;
use configuration::Configuration;

#[derive(Default)]
/// Sell contract canister API
pub struct SellContract;

impl SellContract {
    /// Returns whether caller is custodial of the canister
    pub fn is_custodial(caller: Principal) -> bool {
        Configuration::is_custodial(caller)
    }
}
