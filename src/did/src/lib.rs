//! # DID
//!
//! Did defines the shared types used by the different canisters.

mod common;
pub mod deferred;

pub use common::{
    HttpRequest, HttpResponse, StorableLogSettings, StorableNat, StorablePrincipal, H160, ID,
};
