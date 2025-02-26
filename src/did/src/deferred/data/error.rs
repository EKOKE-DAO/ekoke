use candid::{CandidType, Deserialize};
use ic_cdk::api::call::RejectionCode;
use thiserror::Error;

use crate::ID;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum DeferredDataError {
    #[error("unauthorized caller")]
    Unauthorized,
    #[error("contract error: {0}")]
    Contract(ContractError),
    #[error("real estate error: {0}")]
    RealEstate(RealEstateError),
    #[error("configuration error: {0}")]
    Configuration(#[from] ConfigurationError),
    #[error("storage error")]
    StorageError,
    #[error("inter-canister call error: ({0:?}): {1}")]
    CanisterCall(RejectionCode, String),
    #[error("invalid signature")]
    InvalidSignature,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum ContractError {
    #[error("contract properties should start with 'contract:'")]
    BadContractProperty,
    #[error("the provided contract ID ({0}) doesn't exist in the canister storage")]
    ContractNotFound(ID),
    #[error("document {0} not found")]
    DocumentNotFound(u64),
    #[error("document size mismatch provided size: {0}, actual size: {1}")]
    DocumentSizeMismatch(u64, u64),
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum RealEstateError {
    #[error("the provided real estate ID ({0}) doesn't exist in the canister storage")]
    NotFound(ID),
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum ConfigurationError {
    #[error("the owner cannot be anonymous")]
    AnonymousOwner,
    #[error("the minter cannot be anonymous")]
    AnonymousMinter,
}
