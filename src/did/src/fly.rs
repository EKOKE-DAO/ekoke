//! Types associated to the "Fly" canister

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::icrc1::account::Account;
use thiserror::Error;

use crate::ID;

pub type FlyResult<T> = Result<T, FlyError>;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum FlyError {
    #[error("balance error {0}")]
    Balance(BalanceError),
    #[error("configuration error {0}")]
    Configuration(ConfigurationError),
    #[error("pool error {0}")]
    Pool(PoolError),
    #[error("storage error")]
    StorageError,
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
pub enum PoolError {
    #[error("pool not found for contract {0}")]
    PoolNotFound(ID),
    #[error("not enough tokens in pool")]
    NotEnoughTokens,
}

/// 0.000000000001 $fly
pub type PicoFly = u64;

/// These are the arguments which are taken by the fly canister on init
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct FlyInitData {
    pub admins: Vec<Principal>,
    pub minting_account: Principal,
    /// Total supply of $fly tokens
    pub total_supply: u64,
    /// Initial balances (wallet subaccount -> picofly)
    pub initial_balances: Vec<(Account, PicoFly)>,
    /// Dilazionato canister
    pub dilazionato_canister: Principal,
}

/// Fly user roles. Defines permissions
#[derive(Clone, Copy, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub enum Role {
    /// Administrator
    Admin,
    /// Call reserved to Dilazionato Canister
    DilazionatoCanister,
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
