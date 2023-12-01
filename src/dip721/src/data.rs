use candid::{CandidType, Deserialize, Int, Nat, Principal};
use thiserror::Error;

/// Metadata for a DIP721 canister
#[derive(CandidType, Default, Deserialize)]
pub struct Metadata {
    pub created_at: u64,
    pub custodians: Vec<Principal>,
    pub logo: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub upgraded_at: u64,
}

/// Canister stats
#[derive(CandidType)]
pub struct Stats {
    pub cycles: Nat,
    pub total_supply: Nat,
    pub total_transactions: Nat,
    pub total_unique_holders: Nat,
}

pub type TokenIdentifier = Nat;

/// Properties value representation for a token
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum GenericValue {
    BoolContent(bool),
    TextContent(String),
    BlobContent(Vec<u8>),
    Principal(Principal),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    NatContent(Nat),
    Int8Content(i8),
    Int16Content(i16),
    Int32Content(i32),
    Int64Content(i64),
    IntContent(Int),
    FloatContent(f64), // motoko only support f64
    NestedContent(Vec<(String, GenericValue)>),
}

/// Metadata for a DIP721 token
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenMetadata {
    pub approved_at: Option<u64>,
    pub approved_by: Option<Principal>,
    pub burned_at: Option<u64>,
    pub burned_by: Option<Principal>,
    pub is_burned: bool,
    pub minted_at: u64,
    pub minted_by: Principal,
    pub operator: Option<Principal>,
    pub owner: Option<Principal>,
    pub properties: Vec<(String, GenericValue)>,
    pub token_identifier: TokenIdentifier,
    pub transferred_at: Option<u64>,
    pub transferred_by: Option<Principal>,
}

/// Supported interfaces for a DIP721 canister
#[derive(CandidType, PartialEq, Eq, Debug, Deserialize)]
pub enum SupportedInterface {
    Approval,
    Burn,
    Mint,
    TransactionHistory,
}

/// Represent an NFT error to return via API
#[derive(CandidType, Debug, Deserialize, Clone, PartialEq, Eq, Error)]
pub enum NftError {
    #[error("self transfer is not allowed")]
    SelfTransfer,
    #[error("token not found")]
    TokenNotFound,
    #[error("transaction not found")]
    TxNotFound,
    #[error("not approved")]
    SelfApprove,
    #[error("operator not found")]
    OperatorNotFound,
    #[error("unauthorized owner")]
    UnauthorizedOwner,
    #[error("unauthorized operator")]
    UnauthorizedOperator,
    #[error("NFT existed")]
    ExistedNFT,
    #[error("owner not found")]
    OwnerNotFound,
    #[error("{0}")]
    Other(String),
}

/// Transaction event
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct TxEvent {
    pub caller: Principal,
    pub details: Vec<(String, GenericValue)>,
    pub operation: String,
    pub time: u64,
}
