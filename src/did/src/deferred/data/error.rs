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
    #[error("the provided contract ID ({0}) is closed")]
    ContractIsClosed(ID),
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum ConfigurationError {
    #[error("the owner cannot be anonymous")]
    AnonymousOwner,
    #[error("the minter cannot be anonymous")]
    AnonymousMinter,
}
