use candid::{CandidType, Deserialize};
use ic_cdk::api::call::RejectionCode;
use thiserror::Error;

use crate::deferred::data::DeferredDataError;
use crate::ID;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum DeferredMinterError {
    #[error("unauthorized caller")]
    Unauthorized,
    #[error("contract error: {0}")]
    Contract(ContractError),
    #[error("close contract error: {0}")]
    CloseContract(#[from] CloseContractError),
    #[error("configuration error: {0}")]
    Configuration(#[from] ConfigurationError),
    #[error("storage error")]
    StorageError,
    #[error("inter-canister call error: ({0:?}): {1}")]
    CanisterCall(RejectionCode, String),
    #[error("deferred data canister error: {0}")]
    DataCanister(#[from] DeferredDataError),
    #[error("ecdsa error: {0}")]
    Ecdsa(#[from] EcdsaError),
    #[error("evm rpc error: {0}")]
    EvmRpc(String),
    #[error("failed to decode output: {0}")]
    FailedToDecodeOutput(String),
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum ContractError {
    #[error("contract properties should start with 'contract:'")]
    BadContractProperty,
    #[error("the provided contract expiration is invalid")]
    BadContractExpiration,
    #[error("the provided contract ID ({0}) doesn't exist in the canister storage")]
    ContractNotFound(ID),
    #[error("the contract provided has no tokens")]
    ContractHasNoTokens,
    #[error("the provided contract value is not a multiple of the number of installments")]
    ContractValueIsNotMultipleOfInstallments,
    #[error("the provided contract has no seller")]
    ContractHasNoSeller,
    #[error("the provided contract has no buyer")]
    ContractHasNoBuyer,
    #[error("in order to close the contract, all the tokens must be owned by the seller")]
    CannotCloseContract,
    #[error("the provided contract seller quota sum is not 100")]
    ContractSellerQuotaIsNot100,
    #[error("currency {0} is not allowed for contracts")]
    CurrencyNotAllowed(String),
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum ConfigurationError {
    #[error("there must be at least one custodial")]
    CustodialsCantBeEmpty,
    #[error("the canister custodial cannot be anonymous")]
    AnonymousCustodial,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum CloseContractError {
    #[error("the provided contract ID ({0}) doesn't exist in the canister storage")]
    ContractNotFound(ID),
    #[error("the contract {0} hasn't expired yet")]
    ContractNotExpired(ID),
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum EcdsaError {
    #[error("invalid public key")]
    InvalidPublicKey,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("failed to compute recovery id")]
    RecoveryIdError,
}
