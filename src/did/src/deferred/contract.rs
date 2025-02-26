use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::Serialize;
use time::Date;

pub use crate::ID;

mod generic_value;

pub use self::generic_value::GenericValue;
use super::agency::AgencyId;
use super::{ContractError, DeferredMinterError, DeferredMinterResult};
use crate::H160;

/// A sell contract for a building
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub struct Contract {
    /// Contract ID
    pub id: ID,
    /// Contract type
    pub r#type: ContractType,
    /// The contractors selling the building with their quota
    pub sellers: Vec<Seller>,
    /// Contract buyers. Those who must pay
    pub buyers: Vec<H160>,
    /// Number of installments
    pub installments: u64,
    /// Contract value value
    pub value: u64,
    /// Deposit fiat value (already paid)
    pub deposit: u64,
    /// Currency symbol
    pub currency: String,
    /// Data associated to the contract
    pub properties: ContractProperties,
    /// Restricted data associated to the contract
    pub restricted_properties: RestrictedContractProperties,
    /// Documents associated to the contract
    pub documents: ContractDocuments,
    /// Agency id
    pub agency: AgencyId,
    /// Real estate id
    pub real_estate: ID,
    /// Contract expiration date YYYY-MM-DD
    pub expiration: String,
    /// If the contract is closed
    pub closed: bool,
}

impl Contract {
    /// Check if the given address is a seller
    pub fn is_seller(&self, address: &H160) -> bool {
        self.sellers.iter().any(|s| &s.address == address)
    }

    /// Check if the given address is a buyer
    pub fn is_buyer(&self, address: &H160) -> bool {
        self.buyers.iter().any(|b| b == address)
    }

    /// Get the expiration date of the contract
    pub fn expiration(&self) -> DeferredMinterResult<Date> {
        let format = time::macros::format_description!("[year]-[month]-[day]");

        match time::Date::parse(&self.expiration, format) {
            Ok(expiration) => Ok(expiration),
            Err(_) => Err(DeferredMinterError::Contract(
                ContractError::BadContractExpiration,
            )),
        }
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

/// A list of restricted properties associated to a contract
pub type RestrictedContractProperties = Vec<(String, RestrictedProperty)>;

pub type ContractDocuments = Vec<(u64, ContractDocument)>;

/// A struct which defines a document associated to a contract
///
/// The id must be used to access the document
#[derive(Clone, Debug, CandidType, PartialEq, Serialize, Deserialize)]
pub struct ContractDocument {
    pub access_list: Vec<RestrictionLevel>,
    pub mime_type: String,
    pub name: String,
    pub size: u64,
}

/// A struct which defines a document data
#[derive(Clone, Debug, CandidType, PartialEq, Serialize, Deserialize)]
pub struct ContractDocumentData {
    pub data: Vec<u8>,
    pub mime_type: String,
    pub name: String,
}

/// A restricted property, which defines the access level to the property and its value
#[derive(Clone, Debug, CandidType, PartialEq, Serialize, Deserialize)]
pub struct RestrictedProperty {
    pub access_list: Vec<RestrictionLevel>,
    pub value: GenericValue,
}

/// A variant which defines the restriction level for a contract property
#[derive(Clone, Debug, CandidType, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestrictionLevel {
    /// Seller can access the property
    Seller,
    /// Buyer can access the property
    Buyer,
    /// Agent can access the property
    Agent,
    /// Public can access the property
    Public,
}

/// A variant which defines the contract type
#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContractType {
    Financing,
    Sell,
}

/// A struct which defines the seller of a contract
/// The seller has an Ethereum address [`H160`] and a quota.
/// A contract may have more than one seller and the quota defines the percentage of the contract ownership.
/// The sum of all quotas must be 100.
#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize, Serialize)]
pub struct Seller {
    pub address: H160,
    pub quota: u8,
}

/// Data to be provided to register a contract
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ContractRegistration {
    pub r#type: ContractType,
    /// ID of the real estate associated to the contract
    pub real_estate_id: ID,
    /// Contract sellers. Those who must sell
    pub sellers: Vec<Seller>,
    /// Contract buyers. Those who must pay
    pub buyers: Vec<H160>,
    /// Total Fiat value of the contract (to pay) (`instalments` * `token_value` must be equal to `value`)
    pub value: u64,
    /// Token value
    pub token_value: u64,
    /// Current Fiat value of the contract (to pay) (`instalments` * `token_value` must be equal to `value`)
    pub currency: String,
    /// Deposit value in fiat
    pub deposit: u64,
    /// Must be a divisor of `value`
    pub installments: u64,
    /// Contract expiration date YYYY-MM-DD
    pub expiration: String,
    pub properties: ContractProperties,
    pub restricted_properties: RestrictedContractProperties,
}

impl Default for ContractRegistration {
    fn default() -> Self {
        Self {
            r#type: ContractType::Sell,
            real_estate_id: 0u64.into(),
            sellers: Vec::new(),
            buyers: Vec::new(),
            value: 0,
            token_value: 0,
            currency: String::new(),
            deposit: 0,
            installments: 1,
            expiration: "1970-01-01".to_string(),
            properties: Vec::new(),
            restricted_properties: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {

    use candid::Principal;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_contract() {
        let contract = Contract {
            id: ID::from(1_u64),
            r#type: ContractType::Sell,
            sellers: vec![
                Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 51,
                },
                Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 49,
                },
            ],
            buyers: vec![
                H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A").unwrap(),
                H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A").unwrap(),
            ],
            installments: 2,
            value: 250_000,
            deposit: 50_000,
            currency: "EUR".to_string(),
            documents: vec![],
            properties: vec![(
                "Rome".to_string(),
                GenericValue::TextContent("Rome".to_string()),
            )],
            restricted_properties: vec![(
                "Secret".to_string(),
                RestrictedProperty {
                    access_list: vec![RestrictionLevel::Agent],
                    value: GenericValue::TextContent("Secret".to_string()),
                },
            )],
            agency: Principal::management_canister(),
            real_estate: 1u64.into(),
            expiration: "2040-01-01".to_string(),
            closed: false,
        };
        let data = Encode!(&contract).unwrap();
        let decoded_contract = Decode!(&data, Contract).unwrap();

        assert_eq!(contract.id, decoded_contract.id);
        assert_eq!(contract.sellers, decoded_contract.sellers);
        assert_eq!(contract.buyers, decoded_contract.buyers);
        assert_eq!(contract.properties, decoded_contract.properties);
        assert_eq!(contract.value, decoded_contract.value);
        assert_eq!(contract.currency, decoded_contract.currency);
        assert_eq!(contract.installments, decoded_contract.installments);
        assert_eq!(contract.agency, decoded_contract.agency);
    }
}
