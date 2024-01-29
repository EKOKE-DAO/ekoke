use candid::CandidType;
use ic_cdk::api::call::RejectionCode;
use icrc::{icrc1, icrc2};
use serde::Deserialize;
use thiserror::Error;

use crate::ID;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum EkokeError {
    #[error("allowance error {0}")]
    Allowance(AllowanceError),
    #[error("balance error {0}")]
    Balance(BalanceError),
    #[error("configuration error {0}")]
    Configuration(ConfigurationError),
    #[error("ecdsa error {0}")]
    Ecdsa(EcdsaError),
    #[error("pool error {0}")]
    Pool(PoolError),
    #[error("register error {0}")]
    Register(RegisterError),
    #[error("storage error")]
    StorageError,
    #[error("inter-canister call error: ({0:?}): {1}")]
    CanisterCall(RejectionCode, String),
    #[error("icrc2 transfer error {0:?}")]
    Icrc2Transfer(icrc2::transfer_from::TransferFromError),
    #[error("icrc1 transfer error {0:?}")]
    Icrc1Transfer(icrc1::transfer::TransferError),
    #[error("xrc error")]
    XrcError,
    #[error("eth rpc error: ({0}): {1}")]
    EthRpcError(i32, String),
}

impl From<icrc2::transfer_from::TransferFromError> for EkokeError {
    fn from(value: icrc2::transfer_from::TransferFromError) -> Self {
        Self::Icrc2Transfer(value)
    }
}

impl From<icrc1::transfer::TransferError> for EkokeError {
    fn from(value: icrc1::transfer::TransferError) -> Self {
        Self::Icrc1Transfer(value)
    }
}

impl From<xrc::ExchangeRateError> for EkokeError {
    fn from(_: xrc::ExchangeRateError) -> Self {
        Self::XrcError
    }
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum AllowanceError {
    #[error("allowance not found")]
    AllowanceNotFound,
    #[error("allowance changed")]
    AllowanceChanged,
    #[error("allowance expired")]
    AllowanceExpired,
    #[error("the spender cannot be the caller")]
    BadSpender,
    #[error("the expiration date is in the past")]
    BadExpiration,
    #[error("insufficient funds")]
    InsufficientFunds,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum BalanceError {
    #[error("account not found")]
    AccountNotFound,
    #[error("insufficient balance")]
    InsufficientBalance,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum ConfigurationError {
    #[error("there must be at least one admin")]
    AdminsCantBeEmpty,
    #[error("the canister admin cannot be anonymous")]
    AnonymousAdmin,
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

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum PoolError {
    #[error("pool not found for contract {0}")]
    PoolNotFound(ID),
    #[error("not enough tokens in pool")]
    NotEnoughTokens,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum RegisterError {
    #[error("transaction not found in the register")]
    TransactionNotFound,
}
