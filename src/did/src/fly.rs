//! Types associated to the "Fly" canister

use candid::{CandidType, Deserialize};
use thiserror::Error;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum FlyError {
    #[error("storage error")]
    StorageError,
}
