//! # DID
//!
//! Did defines the shared types used by the different canisters.

mod common;

pub mod deferred;
pub mod ekoke;
pub mod ekoke_archive;
pub mod ekoke_index;
pub mod marketplace;
pub use common::{
    HttpApiRequest, HttpRequest, HttpResponse, StorableAccount, StorableNat, StorablePrincipal,
    H160, ID,
};
