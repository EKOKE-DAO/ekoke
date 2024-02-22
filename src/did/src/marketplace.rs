//! Types associated to the "Ekoke" canister

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use dip721::NftError;
use ic_cdk::api::call::RejectionCode;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::{icrc1, icrc2};
use thiserror::Error;

use crate::deferred::DeferredError;
use crate::ekoke::EkokeError;

pub type MarketplaceResult<T> = Result<T, MarketplaceError>;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum MarketplaceError {
    #[error("configuration error {0}")]
    Configuration(ConfigurationError),
    #[error("storage error")]
    StorageError,
    #[error("ekoke canister error {0}")]
    EkokeCanister(#[from] EkokeError),
    #[error("deferred canister error {0}")]
    DeferredCanister(#[from] DeferredError),
    #[error("dip721 error {0}")]
    Dip721(#[from] NftError),
    #[error("inter-canister call error: ({0:?}): {1}")]
    CanisterCall(RejectionCode, String),
    #[error("icrc2 transfer error {0:?}")]
    Icrc2Transfer(icrc2::transfer_from::TransferFromError),
    #[error("icrc1 transfer error {0:?}")]
    Icrc1Transfer(icrc1::transfer::TransferError),
    #[error("xrc error: {0}")]
    XrcError(String),
    #[error("token not found")]
    TokenNotFound,
    #[error("buy error: {0}")]
    Buy(#[from] BuyError),
}

impl From<icrc2::transfer_from::TransferFromError> for MarketplaceError {
    fn from(value: icrc2::transfer_from::TransferFromError) -> Self {
        Self::Icrc2Transfer(value)
    }
}

impl From<icrc1::transfer::TransferError> for MarketplaceError {
    fn from(value: icrc1::transfer::TransferError) -> Self {
        Self::Icrc1Transfer(value)
    }
}

impl From<xrc::ExchangeRateError> for MarketplaceError {
    fn from(e: xrc::ExchangeRateError) -> Self {
        Self::XrcError(format!("{e:?}"))
    }
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum ConfigurationError {
    #[error("there must be at least one admin")]
    AdminsCantBeEmpty,
    #[error("the canister admin cannot be anonymous")]
    AnonymousAdmin,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum BuyError {
    #[error("token has no owner")]
    TokenHasNoOwner,
    #[error("caller already owns token")]
    CallerAlreadyOwnsToken,
    #[error("ICP allowance has expired")]
    IcpAllowanceExpired,
    #[error("ICP allowance is not enough")]
    IcpAllowanceNotEnough,
}

/// These are the arguments which are taken by the marketplace canister on init
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct MarketplaceInitData {
    pub admins: Vec<Principal>,
    /// Deferred canister
    pub deferred_canister: Principal,
    /// Ekoke canister
    pub ekoke_ledger_canister: Principal,
    /// Ekoke liquidity pool canister
    pub ekoke_liquidity_pool_canister: Principal,
    /// ICP ledger canister
    pub icp_ledger_canister: Principal,
    /// XRC canister
    pub xrc_canister: Principal,
}

/// Marketplace user roles. Defines permissions
#[derive(Clone, Copy, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub enum Role {
    /// Administrator
    Admin,
}

impl Storable for Role {
    const BOUND: Bound = Bound::Bounded {
        max_size: 8,
        is_fixed_size: true,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Role).unwrap()
    }
}

/// List of roles
#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub struct Roles(pub Vec<Role>);

impl From<Vec<Role>> for Roles {
    fn from(roles: Vec<Role>) -> Self {
        Self(roles)
    }
}

impl Storable for Roles {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Vec<Role>).unwrap().into()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_role() {
        let role: Roles = vec![Role::Admin].into();

        let data = role.to_bytes();
        let decoded_role = Roles::from_bytes(data);
        assert_eq!(role, decoded_role);
    }
}
