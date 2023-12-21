//! # DID
//!
//! Did defines the shared types used by the different canisters.

mod common;

pub mod deferred;
pub mod fly;
pub mod marketplace;
pub use common::{StorableNat, StorablePrincipal, ID};
