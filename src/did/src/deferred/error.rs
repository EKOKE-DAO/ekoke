use candid::{CandidType, Deserialize};
use dip721::{NftError, TokenIdentifier};
use thiserror::Error;

use crate::fly::FlyError;
use crate::ID;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum DeferredError {
    #[error("unauthorized caller")]
    Unauthorized,
    #[error("fly error: {0}")]
    Fly(#[from] FlyError),
    #[error("token error: {0}")]
    Token(TokenError),
    #[error("configuration error: {0}")]
    Configuration(ConfigurationError),
    #[error("storage error")]
    StorageError,
    #[error("nft error: {0}")]
    Nft(#[from] NftError),
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum TokenError {
    #[error("contract properties should start with 'contract:'")]
    BadContractProperty,
    #[error("the provided contract ID ({0}) already exists in the canister storage")]
    ContractAlreadyExists(ID),
    #[error("the provided contract ID ({0}) is already signed")]
    ContractAlreadySigned(ID),
    #[error("the provided contract ID ({0}) is not signed")]
    ContractNotSigned(ID),
    #[error("the provided contract ID should be empty on register")]
    ContractTokensShouldBeEmpty,
    #[error("the provided contract ID ({0}) doesn't exist in the canister storage")]
    ContractNotFound(ID),
    #[error("the provided token ID ({0}) already exists in the canister storage")]
    TokenAlreadyExists(TokenIdentifier),
    #[error("the provided token ({0}) doesn't belong to the provided contract")]
    TokenDoesNotBelongToContract(TokenIdentifier),
    #[error("the token {0} owner should be the seller on mint")]
    BadMintTokenOwner(TokenIdentifier),
    #[error("the token defined in the contract differ from the provided tokens")]
    TokensMismatch,
    #[error("the contract provided has no tokens")]
    ContractHasNoTokens,
    #[error("the provided token ID ({0}) doesn't exist in the canister storage")]
    TokenNotFound(TokenIdentifier),
    #[error("the provided token ID ({0}) is burned, so it cannot be touched by any operation")]
    TokenIsBurned(TokenIdentifier),
    #[error("the provided contract value is not a multiple of the number of installments")]
    ContractValueIsNotMultipleOfInstallments,
    #[error("the provided contract has no seller")]
    ContractHasNoSeller,
    #[error("in order to close the contract, all the tokens must be owned by the seller")]
    CannotCloseContract,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum ConfigurationError {
    #[error("there must be at least one custodial")]
    CustodialsCantBeEmpty,
    #[error("the canister custodial cannot be anonymous")]
    AnonymousCustodial,
}
