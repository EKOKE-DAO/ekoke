use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use dip721::{GenericValue, TokenIdentifier};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;

use crate::fly::PicoFly;
use crate::ID;

/// A sell contract for a building
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Contract {
    /// Contract ID
    pub id: ID,
    /// Contract type
    pub r#type: ContractType,
    /// The contractors selling the building with their quota
    pub seller: Principal,
    /// Contract buyers. Those who must pay
    pub buyers: Vec<Principal>,
    /// Tokens associated to the contract, by id
    pub tokens: Vec<TokenIdentifier>,
    /// Number of installments
    pub installments: u64,
    /// Whether the contract is signed. Tokens are minted only if the contract is signed
    pub is_signed: bool,
    /// Initial Fiat value of the contract
    pub initial_value: u64,
    /// Current Fiat value of the contract (to pay)
    pub value: u64,
    /// Currency symbol
    pub currency: String,
    /// Data associated to the contract
    pub properties: ContractProperties,
}

impl Storable for Contract {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

/// A list of properties associated to a contract
pub type ContractProperties = Vec<(String, GenericValue)>;

/// A variant which defines the contract type
#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ContractType {
    Financing,
    Sell,
}

/// A Non fungible token related to an installment of a contract
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Token {
    /// Unique identifier of the token
    pub id: TokenIdentifier,
    /// Contract id
    pub contract_id: ID,
    /// Token owner. If none the token is burned
    pub owner: Option<Principal>,
    /// Value of the single token (FIAT)
    pub value: u64,
    /// $picoFly (pico-fly) reward for buying a Token
    pub picofly_reward: PicoFly,
    /// A principal who can operate on the token
    pub operator: Option<Principal>,
    /// Whether the token is burned
    pub is_burned: bool,
    /// Timestamp the token was minted at
    pub minted_at: u64,
    /// Principal who minted the token
    pub minted_by: Principal,
    /// Timestamp the token was approved at
    pub approved_at: Option<u64>,
    /// Principal who approved the token
    pub approved_by: Option<Principal>,
    /// Timestamp the token was burned at
    pub burned_at: Option<u64>,
    /// Principal who burned the token
    pub burned_by: Option<Principal>,
    /// Timestamp the token was transferred at
    pub transferred_at: Option<u64>,
    /// Principal who transferred the token
    pub transferred_by: Option<Principal>,
}

impl Storable for Token {
    const BOUND: Bound = Bound::Bounded {
        max_size: 512,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

/// Data to be provided to register a contract
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ContractRegistration {
    pub id: ID,
    pub r#type: ContractType,
    pub seller: Principal,
    pub buyers: Vec<Principal>,
    pub value: u64,
    pub currency: String,
    pub installments: u64,
    pub properties: ContractProperties,
}
