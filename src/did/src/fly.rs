//! Types associated to the "Fly" canister

use candid::{CandidType, Decode, Deserialize, Encode, Nat, Principal};
use ic_cdk::api::call::RejectionCode;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::icrc1::account::Account;
use icrc::icrc1::transfer::Memo;
use icrc::{icrc1, icrc2};
use thiserror::Error;

use crate::{H160, ID};

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
    #[error("inter-canister call error: ({0:?}): {1}")]
    CanisterCall(RejectionCode, String),
    #[error("icrc2 transfer error {0:?}")]
    Icrc2Transfer(icrc2::transfer_from::TransferFromError),
    #[error("icrc1 transfer error {0:?}")]
    Icrc1Transfer(icrc1::transfer::TransferError),
    #[error("xrc error")]
    XrcError,
}

impl From<icrc2::transfer_from::TransferFromError> for FlyError {
    fn from(value: icrc2::transfer_from::TransferFromError) -> Self {
        Self::Icrc2Transfer(value)
    }
}

impl From<icrc1::transfer::TransferError> for FlyError {
    fn from(value: icrc1::transfer::TransferError) -> Self {
        Self::Icrc1Transfer(value)
    }
}

impl From<xrc::ExchangeRateError> for FlyError {
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
    /// The canister ID of the CKBTC canister
    pub ckbtc_canister: Principal,
    /// The canister ID of the CKETH ledger canister
    pub cketh_ledger_canister: Principal,
    /// The canister ID of the CKETH minter canister
    pub cketh_minter_canister: Principal,
    /// The Ethereum address of the ERC20 bridge
    pub erc20_bridge_address: H160,
    /// Initial ERC20 swap fee
    pub erc20_swap_fee: u64,
    /// Total supply of $picofly tokens
    pub total_supply: PicoFly,
    /// Initial balances (wallet subaccount -> picofly)
    pub initial_balances: Vec<(Account, PicoFly)>,
    /// Deferred canister
    pub deferred_canister: Principal,
    /// ICP ledger canister
    pub icp_ledger_canister: Principal,
    /// Marketplace canister
    pub marketplace_canister: Principal,
    /// Swap account
    pub swap_account: Account,
    /// Minting account, the account that can mint new tokens and burn them
    pub minting_account: Account,
    /// XRC canister
    pub xrc_canister: Principal,
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

/// The accounts that hold the liquidity pools for the CKBTC and ICP tokens.
#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub struct LiquidityPoolAccounts {
    /// The account that holds the pool for the CKBTC token.
    pub ckbtc: Account,
    /// The account that holds the pool for the ICP tokens.
    pub icp: Account,
}

/// The balance of the liquidity pool
#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub struct LiquidityPoolBalance {
    /// CKBTC tokens hold in the liquidity pool
    pub ckbtc: Nat,
    /// ICP tokens hold in the liquidity pool
    pub icp: Nat,
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
