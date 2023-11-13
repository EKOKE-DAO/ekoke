//! Types associated to the "Fly" canister

use candid::{CandidType, Deserialize};
use thiserror::Error;

use crate::ID;

pub type FlyResult<T> = Result<T, FlyError>;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum FlyError {
    #[error("pool error {0}")]
    Pool(PoolError),
    #[error("storage error")]
    StorageError,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum PoolError {
    #[error("pool not found for contract {0}")]
    PoolNotFound(ID),
    #[error("not enough tokens in pool")]
    NotEnoughTokens,
}
