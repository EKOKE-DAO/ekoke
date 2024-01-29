//! # DID
//!
//! Did defines the shared types used by the different canisters.

mod common;

pub mod deferred;
pub mod ekoke;
pub mod marketplace;
pub use common::{StorableAccount, StorableNat, StorablePrincipal, ID};
