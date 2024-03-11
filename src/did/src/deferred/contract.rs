use candid::{CandidType, Decode, Deserialize, Encode, Principal};
pub use dip721::{GenericValue, TokenIdentifier};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::Serialize;
use time::Date;

pub use crate::ID;

mod agency;
mod info;
mod token;

pub use agency::{Agency, Continent};
pub use info::TokenInfo;
pub use token::Token;

use super::{DeferredError, DeferredResult, TokenError};

/// A sell contract for a building
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Contract {
    /// Contract ID
    pub id: ID,
    /// Contract type
    pub r#type: ContractType,
    /// The contractors selling the building with their quota
    pub sellers: Vec<Seller>,
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
    /// Agency data
    pub agency: Option<Agency>,
    /// Contract expiration date YYYY-MM-DD
    pub expiration: Option<String>,
}

impl Contract {
    pub fn is_seller(&self, principal: &Principal) -> bool {
        self.sellers.iter().any(|s| &s.principal == principal)
    }

    pub fn expiration(&self) -> Option<DeferredResult<Date>> {
        let format = time::macros::format_description!("[year]-[month]-[day]");
        self.expiration
            .as_deref()
            .map(|expiration| match time::Date::parse(expiration, format) {
                Ok(expiration) => Ok(expiration),
                Err(_) => Err(DeferredError::Token(TokenError::BadContractExpiration)),
            })
    }
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
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum ContractType {
    Financing,
    Sell,
}

/// A variant which defines a contract seller.
/// A contract may have more than one seller and the quota defines the percentage of the contract ownership.
/// The sum of all quotas must be 100.
#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize, Serialize)]
pub struct Seller {
    pub principal: Principal,
    pub quota: u8,
}

/// Data to be provided to register a contract
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ContractRegistration {
    pub r#type: ContractType,
    pub sellers: Vec<Seller>,
    pub buyers: Vec<Principal>,
    pub value: u64,
    pub currency: String,
    pub installments: u64,
    pub expiration: Option<String>,
    pub properties: ContractProperties,
}
