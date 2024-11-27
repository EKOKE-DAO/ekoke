//! Types associated to the "Deferred" canister

mod contract;
mod data;
mod minter;

pub type DeferredMinterResult<T> = Result<T, DeferredMinterError>;
pub type DeferredDataResult<T> = Result<T, DeferredDataError>;

pub use self::contract::{
    Agency, Continent, Contract, ContractDocument, ContractDocumentData, ContractDocuments,
    ContractProperties, ContractRegistration, ContractType, GenericValue,
    RestrictedContractProperties, RestrictedProperty, RestrictionLevel, Seller, ID,
};
pub use self::data::{
    ConfigurationError as DataConfigurationError, ContractError as DataContractError,
    DeferredDataError, DeferredDataInitData,
};
pub use self::minter::{
    CloseContractError, ConfigurationError, ContractError, DeferredMinterError,
    DeferredMinterInitData, EcdsaError, EcdsaKey, Role, Roles,
};

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use candid::{Decode, Encode, Principal};
    use ic_stable_structures::Storable as _;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{H160, ID};

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
            documents: HashMap::default(),
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
            agency: Some(Agency {
                name: "Agency".to_string(),
                address: "Address".to_string(),
                city: "City".to_string(),
                region: "Region".to_string(),
                zip_code: "Zip".to_string(),
                country: "Country".to_string(),
                continent: Continent::Europe,
                email: "Email".to_string(),
                website: "Website".to_string(),
                mobile: "Mobile".to_string(),
                vat: "VAT".to_string(),
                agent: "Agent".to_string(),
                logo: None,
                owner: Principal::anonymous(),
            }),
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

    #[test]
    fn test_should_encode_role() {
        let role: Roles = vec![Role::Agent, Role::Custodian].into();

        let data = role.to_bytes();
        let decoded_role = Roles::from_bytes(data);
        assert_eq!(role, decoded_role);
    }
}
