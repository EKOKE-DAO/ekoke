mod error;

use std::fmt;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_log::LogSettingsV2;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;

pub use self::error::{
    CloseContractError, ConfigurationError, ContractError, DeferredMinterError, EcdsaError,
};
use crate::H160;

/// These are the arguments which are taken by the deferred minter canister at creation
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct DeferredMinterInitData {
    /// The list of allowed currencies
    pub allowed_currencies: Vec<String>,
    /// Ethereum chain id
    pub chain_id: u64,
    /// Principal of the custodians
    pub custodians: Vec<Principal>,
    /// Principal of deferred-data canister
    pub deferred_data: Principal,
    /// Ethereum address of deferred-erc721 contract
    pub deferred_erc721: H160,
    /// ethereum ecdsa key
    pub ecdsa_key: EcdsaKey,
    /// Principal of evm-rpc canister
    pub evm_rpc: Principal,
    /// Custom evm rpc api
    pub evm_rpc_api: Option<String>,
    /// Log settings
    pub log_settings: LogSettingsV2,
    /// Ethereum address of reward pool contract
    pub reward_pool: H160,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, CandidType, Deserialize, PartialEq, Eq)]
pub enum EcdsaKey {
    Dfx = 0,
    Test = 1,
    Production = 2,
}

impl From<u8> for EcdsaKey {
    fn from(value: u8) -> Self {
        match value {
            0 => EcdsaKey::Dfx,
            1 => EcdsaKey::Test,
            2 => EcdsaKey::Production,
            _ => panic!("Invalid EcdsaKey value"),
        }
    }
}

impl fmt::Display for EcdsaKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EcdsaKey::Dfx => write!(f, "dfx_test_key"),
            EcdsaKey::Test => write!(f, "test_key_1"),
            EcdsaKey::Production => write!(f, "key_1"),
        }
    }
}

/// Deferred user roles. Defines permissions
#[derive(Clone, Copy, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub enum Role {
    /// Administrator, follows DIP721 standard
    Custodian,
    /// A user who can create contracts, but cannot sign them
    Agent,
    /// A user who can set the gas price
    GasStation,
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
