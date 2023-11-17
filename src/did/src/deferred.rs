//! Types associated to the "Sell Contract" canister

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use dip721::{GenericValue, TokenIdentifier, TxEvent};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use thiserror::Error;

use crate::fly::{FlyError, PicoFly};
use crate::ID;

pub type DeferredResult<T> = Result<T, DeferredError>;

/// These are the arguments which are taken by the sell contract canister on init
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct DeferredInitData {
    pub custodians: Vec<Principal>,
    pub fly_canister: Principal,
    pub marketplace_canister: Principal,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum DeferredError {
    #[error("unauthorized caller")]
    Unauthorized,
    #[error("fly error: {0}")]
    Fly(#[from] FlyError),
    #[error("token error: {0}")]
    Token(TokenError),
    #[error("configuration error: {0}")]
    Configuration(ConfigurationError),
    #[error("storage error")]
    StorageError,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum TokenError {
    #[error("contract properties should start with 'contract:'")]
    BadContractProperty,
    #[error("the provided contract ID ({0}) already exists in the canister storage")]
    ContractAlreadyExists(ID),
    #[error("the provided contract ID ({0}) is already signed")]
    ContractAlreadySigned(ID),
    #[error("the provided contract ID ({0}) is not signed")]
    ContractNotSigned(ID),
    #[error("the provided contract ID should be empty on register")]
    ContractTokensShouldBeEmpty,
    #[error("the provided contract ID ({0}) doesn't exist in the canister storage")]
    ContractNotFound(ID),
    #[error("the provided token ID ({0}) already exists in the canister storage")]
    TokenAlreadyExists(TokenIdentifier),
    #[error("the provided token ({0}) doesn't belong to the provided contract")]
    TokenDoesNotBelongToContract(TokenIdentifier),
    #[error("the token {0} owner should be the seller on mint")]
    BadMintTokenOwner(TokenIdentifier),
    #[error("the token defined in the contract differ from the provided tokens")]
    TokensMismatch,
    #[error("the contract provided has no tokens")]
    ContractHasNoTokens,
    #[error("the provided token ID ({0}) doesn't exist in the canister storage")]
    TokenNotFound(TokenIdentifier),
    #[error("the provided token ID ({0}) is burned, so it cannot be touched by any operation")]
    TokenIsBurned(TokenIdentifier),
    #[error("the provided contract value is not a multiple of the number of installments")]
    ContractValueIsNotMultipleOfInstallments,
    #[error("the provided expiration date is invalid. It must have syntax YYYY-MM-DD")]
    InvalidExpirationDate,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum ConfigurationError {
    #[error("there must be at least one custodial")]
    CustodialsCantBeEmpty,
    #[error("the canister custodial cannot be anonymous")]
    AnonymousCustodial,
}

/// A sell contract for a building
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Contract {
    /// Contract ID
    pub id: ID,
    /// Contract type
    pub r#type: ContractType,
    /// The contractor selling the building
    pub seller: Principal,
    /// Contract buyers. Those who must pay
    pub buyers: Vec<Principal>,
    /// Contract expiration date
    pub expiration: String,
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

/// Storable TxEvent DIP721 transaction
pub struct StorableTxEvent(pub TxEvent);

impl StorableTxEvent {
    pub fn as_tx_event(&self) -> &TxEvent {
        &self.0
    }
}

impl From<TxEvent> for StorableTxEvent {
    fn from(tx_event: TxEvent) -> Self {
        Self(tx_event)
    }
}

impl Storable for StorableTxEvent {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self.0).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, TxEvent).unwrap().into()
    }
}

/// Data to be provided to register a contract
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ContractRegistration {
    pub id: ID,
    pub r#type: ContractType,
    pub seller: Principal,
    pub buyers: Vec<Principal>,
    pub expiration: String,
    pub value: u64,
    pub currency: String,
    pub installments: u64,
    pub properties: ContractProperties,
}

/// Deferred user roles. Defines permissions
#[derive(Clone, Copy, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub enum Role {
    /// Administrator, follows DIP721 standard
    Custodian,
    /// A user who can create contracts, but cannot sign them
    Agent,
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
    fn test_should_encode_token() {
        let token = Token {
            id: TokenIdentifier::from(1),
            contract_id: ID::from(1),
            owner: Some(
                Principal::from_text(
                    "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
                )
                .unwrap(),
            ),
            picofly_reward: 4_000_u64.into(),
            transferred_at: None,
            transferred_by: None,
            approved_at: None,
            approved_by: None,
            burned_at: None,
            burned_by: None,
            minted_at: 0,
            minted_by: Principal::anonymous(),
            operator: None,
            value: 100,
            is_burned: false,
        };
        let data = Encode!(&token).unwrap();
        let decoded_token = Decode!(&data, Token).unwrap();

        assert_eq!(token.id, decoded_token.id);
        assert_eq!(token.contract_id, decoded_token.contract_id);
        assert_eq!(token.owner, decoded_token.owner);
        assert_eq!(token.value, decoded_token.value);
        assert_eq!(token.picofly_reward, decoded_token.picofly_reward);
    }

    #[test]
    fn test_should_encode_contract() {
        let contract = Contract {
            id: ID::from(1),
            r#type: ContractType::Sell,
            seller: Principal::from_text(
                "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
            )
            .unwrap(),
            buyers: vec![
                Principal::anonymous(),
                Principal::from_text(
                    "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
                )
                .unwrap(),
            ],
            expiration: "2021-12-31".to_string(),
            installments: 2,
            is_signed: true,
            tokens: vec![TokenIdentifier::from(1), TokenIdentifier::from(2)],
            initial_value: 250_000,
            value: 250_000,
            currency: "EUR".to_string(),
            properties: vec![(
                "Rome".to_string(),
                GenericValue::TextContent("Rome".to_string()),
            )],
        };
        let data = Encode!(&contract).unwrap();
        let decoded_contract = Decode!(&data, Contract).unwrap();

        assert_eq!(contract.id, decoded_contract.id);
        assert_eq!(contract.seller, decoded_contract.seller);
        assert_eq!(contract.buyers, decoded_contract.buyers);
        assert_eq!(contract.expiration, decoded_contract.expiration);
        assert_eq!(contract.tokens, decoded_contract.tokens);
        assert_eq!(contract.properties, decoded_contract.properties);
        assert_eq!(contract.value, decoded_contract.value);
        assert_eq!(contract.initial_value, decoded_contract.initial_value);
        assert_eq!(contract.currency, decoded_contract.currency);
        assert_eq!(contract.installments, decoded_contract.installments);
        assert_eq!(contract.is_signed, decoded_contract.is_signed);
    }

    #[test]
    fn test_should_encode_tx_event() {
        let tx_event: StorableTxEvent = TxEvent {
            caller: Principal::from_text(
                "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
            )
            .unwrap(),
            details: vec![],
            operation: "mint".to_string(),
            time: 0,
        }
        .into();

        let data = tx_event.to_bytes();
        let decoded_tx_event = StorableTxEvent::from_bytes(data);
        assert_eq!(tx_event.0, decoded_tx_event.0);
    }

    #[test]
    fn test_should_encode_role() {
        let role: Roles = vec![Role::Agent, Role::Custodian].into();

        let data = role.to_bytes();
        let decoded_role = Roles::from_bytes(data);
        assert_eq!(role, decoded_role);
    }
}
