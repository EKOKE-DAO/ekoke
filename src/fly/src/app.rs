//! # App
//!
//! API implementation for dilazionato canister

use did::{fly::FlyResult, ID};

mod memory;
mod pool;

use self::pool::Pool;

pub struct FlyCanister;

impl FlyCanister {
    pub fn init() {}

    pub fn post_upgrade() {}

    /// Reserve a pool for the provided contract ID with the provided amount of $mFly tokens
    pub fn reserve_pool(contract_id: ID, picofly_amount: u64) -> FlyResult<u64> {
        Pool::reserve(&contract_id, picofly_amount)
    }
}
