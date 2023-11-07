use std::collections::HashSet;

use candid::{CandidType, Deserialize, Int, Nat, Principal};

/// Metadata for a DIP721 canister
#[derive(CandidType, Default, Deserialize)]
pub struct Metadata {
    pub created_at: u64,
    pub custodians: HashSet<Principal>,
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
#[derive(CandidType, Deserialize)]
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
#[derive(CandidType, Deserialize)]
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
#[derive(CandidType)]
pub enum SupportedInterface {
    Approval,
    Burn,
    Mint,
    TransactionHistory,
}

/// Represent an NFT error to return via API
#[derive(CandidType)]
pub enum NftError {
    SelfTransfer,
    TokenNotFound,
    TxNotFound,
    SelfApprove,
    OperatorNotFound,
    UnauthorizedOwner,
    UnauthorizedOperator,
    ExistedNFT,
    OwnerNotFound,
    Other(String),
}

/// Transaction event
#[derive(CandidType, Deserialize)]
pub struct TxEvent {
    pub caller: Principal,
    pub details: Vec<(String, GenericValue)>,
    pub operation: String,
    pub time: u64,
}
