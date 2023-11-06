//! Types associated to the "Sell Contract" canister

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use thiserror::Error;

use crate::ID;

pub type SellContractResult<T> = Result<T, SellContractError>;

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum SellContractError {
    #[error("mint error: {0}")]
    Mint(MintError),
    #[error("configuration error: {0}")]
    Configuration(ConfigurationError),
    #[error("storage error")]
    StorageError,
}

#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum MintError {
    #[error("the provided contract ID ({0}) already exists in the canister storage")]
    ContractAlreadyExists(ID),
    #[error("the provided token ID ({0}) already exists in the canister storage")]
    TokenAlreadyExists(ID),
    #[error("the provided token ({0}) doesn't belong to the provided contract")]
    TokenDoesNotBelongToContract(ID),
    #[error("the token {0} owner should be the seller on mint")]
    BadMintTokenOwner(ID),
    #[error("the token defined in the contract differ from the provided tokens")]
    TokensMismatch,
    #[error("the contract provided has no tokens")]
    ContractHasNoTokens,
    #[error("the provided contract ID ({0}) doesn't exist in the canister storage")]
    TokenNotFound(ID),
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
    /// The contractor selling the building
    pub seller: Principal,
    /// Contract buyers. Those who must pay
    pub buyers: Vec<Principal>,
    /// Contract expiration date
    pub expiration: String,
    /// Tokens associated to the contract, by id
    pub tokens: Vec<ID>,
    /// $mFLY (milli-fly) reward for buying a Token
    pub mfly_reward: u64,
    /// Fiat value of the contract
    pub value: u64,
    /// Data associated to the building
    pub building: BuildingData,
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

/// A Non fungible token related to an installment of a contract
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Token {
    /// Unique identifier of the token
    pub id: ID,
    /// Contract id
    pub contract_id: ID,
    /// Token owner
    pub owner: Principal,
    /// Value of the single token (FIAT)
    pub value: u64,
    /// Token locked status
    pub locked: bool,
}

impl Storable for Token {
    const BOUND: Bound = Bound::Bounded {
        max_size: 163,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

/// Data associated to a building
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct BuildingData {
    /// The city the building is located at
    pub city: String,
}

impl Storable for BuildingData {
    const BOUND: Bound = Bound::Bounded {
        max_size: 256,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_token() {
        let token = Token {
            id: ID::random(),
            contract_id: ID::random(),
            owner: Principal::from_text(
                "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
            )
            .unwrap(),
            value: 100,
            locked: false,
        };
        let data = Encode!(&token).unwrap();
        let decoded_token = Decode!(&data, Token).unwrap();

        assert_eq!(token.id, decoded_token.id);
        assert_eq!(token.contract_id, decoded_token.contract_id);
        assert_eq!(token.owner, decoded_token.owner);
        assert_eq!(token.value, decoded_token.value);
    }

    #[test]
    fn test_should_encode_building_data() {
        let building_data = BuildingData {
            city: "Rome".to_string(),
        };
        let data = Encode!(&building_data).unwrap();
        let decoded_building_data = Decode!(&data, BuildingData).unwrap();

        assert_eq!(building_data.city, decoded_building_data.city);
    }

    #[test]
    fn test_should_encode_contract() {
        let contract = Contract {
            id: ID::random(),
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
            tokens: vec![ID::random(), ID::random()],
            mfly_reward: 4_000,
            value: 250_000,
            building: BuildingData {
                city: "Rome".to_string(),
            },
        };
        let data = Encode!(&contract).unwrap();
        let decoded_contract = Decode!(&data, Contract).unwrap();

        assert_eq!(contract.id, decoded_contract.id);
        assert_eq!(contract.seller, decoded_contract.seller);
        assert_eq!(contract.buyers, decoded_contract.buyers);
        assert_eq!(contract.expiration, decoded_contract.expiration);
        assert_eq!(contract.tokens, decoded_contract.tokens);
        assert_eq!(contract.mfly_reward, decoded_contract.mfly_reward);
        assert_eq!(contract.building.city, decoded_contract.building.city);
        assert_eq!(contract.value, decoded_contract.value);
    }
}
