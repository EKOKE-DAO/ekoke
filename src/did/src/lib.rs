//! # DID
//!
//! Did defines the shared types used by the different canisters.

mod common;

pub mod deferred;
pub mod ekoke;
pub mod ekoke_erc20_swap;
pub mod ekoke_liquidity_pool;
pub mod ekoke_reward_pool;
pub mod marketplace;
pub use common::{
    HttpApiRequest, HttpRequest, HttpResponse, StorableAccount, StorableNat, StorablePrincipal,
    H160, ID,
};
