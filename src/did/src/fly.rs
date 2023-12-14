//! Types associated to the "Fly" canister

use candid::{CandidType, Decode, Deserialize, Encode, Nat, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::icrc1::account::Account;
use icrc::icrc1::transfer::Memo;
use thiserror::Error;

use crate::ID;

pub type FlyResult<T> = Result<T, FlyError>;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum FlyError {
    #[error("allowance error {0}")]
    Allowance(AllowanceError),
    #[error("balance error {0}")]
    Balance(BalanceError),
    #[error("configuration error {0}")]
    Configuration(ConfigurationError),
    #[error("pool error {0}")]
    Pool(PoolError),
    #[error("register error {0}")]
    Register(RegisterError),
    #[error("storage error")]
    StorageError,
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

/// 0.000000000001 $fly
pub type PicoFly = Nat;

/// These are the arguments which are taken by the fly canister on init
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct FlyInitData {
    pub admins: Vec<Principal>,
    /// Total supply of $fly tokens
    pub total_supply: u64,
    /// Initial balances (wallet subaccount -> picofly)
    pub initial_balances: Vec<(Account, PicoFly)>,
    /// Deferred canister
    pub deferred_canister: Principal,
    /// Marketplace canister
    pub marketplace_canister: Principal,
}

/// Fly user roles. Defines permissions
#[derive(Clone, Copy, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub enum Role {
    /// Administrator
    Admin,
    /// Call reserved to Deferred Canister
    DeferredCanister,
    /// Call reserved to the marketplace
    MarketplaceCanister,
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

#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub struct Transaction {
    pub from: Account,
    pub to: Account,
    pub amount: PicoFly,
    pub fee: PicoFly,
    pub memo: Option<Memo>,
    pub created_at: u64,
}

impl Storable for Transaction {
    const BOUND: Bound = Bound::Bounded {
        max_size: 256,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Transaction).unwrap()
    }
}

#[cfg(test)]
mod test {

    use icrc::icrc1::account::Account;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_role() {
        let role: Roles = vec![Role::Admin].into();

        let data = role.to_bytes();
        let decoded_role = Roles::from_bytes(data);
        assert_eq!(role, decoded_role);
    }

    #[test]
    fn test_should_encode_transaction() {
        let tx = Transaction {
            from: Account {
                owner: Principal::management_canister(),
                subaccount: Some([1u8; 32]),
            },
            to: Account {
                owner: Principal::management_canister(),
                subaccount: None,
            },
            amount: 100_u64.into(),
            fee: 1_u64.into(),
            memo: None,
            created_at: 0,
        };

        let data = tx.to_bytes();
        let decoded_tx = Transaction::from_bytes(data);
        assert_eq!(tx, decoded_tx);
    }
}
