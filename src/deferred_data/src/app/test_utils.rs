use candid::Principal;
use did::deferred::{Agency, Contract, GenericValue, RestrictedProperty, RestrictionLevel, Seller};
use did::H160;

use super::storage::ContractStorage;

pub fn mock_contract(id: u64, installments: u64) -> Contract {
    Contract {
        id: id.into(),
        r#type: did::deferred::ContractType::Financing,
        sellers: vec![Seller {
            address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A").unwrap(),
            quota: 100,
        }],
        buyers: vec![H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A").unwrap()],
        installments,
        value: 250_000,
        deposit: 50_000,
        currency: "EUR".to_string(),
        properties: vec![(
            "contract:city".to_string(),
            GenericValue::TextContent("Rome".to_string()),
        )],
        restricted_properties: vec![(
            "contract:seller_address".to_string(),
            RestrictedProperty {
                access_list: vec![RestrictionLevel::Agent, RestrictionLevel::Seller],
                value: GenericValue::TextContent("Via Roma 123".to_string()),
            },
        )],
        documents: vec![],
        agency: Some(mock_agency()),
        expiration: "2078-01-01".to_string(),
        closed: false,
    }
}

pub fn mock_agency() -> Agency {
    Agency {
        name: "Dummy Real estate".to_string(),
        address: "Via Delle Botteghe Scure".to_string(),
        city: "Rome".to_string(),
        region: "Lazio".to_string(),
        zip_code: "00100".to_string(),
        country: "Italy".to_string(),
        continent: did::deferred::Continent::Europe,
        email: "email".to_string(),
        website: "website".to_string(),
        lat: None,
        lng: None,
        mobile: "mobile".to_string(),
        vat: "vat".to_string(),
        agent: "agent".to_string(),
        logo: None,
        owner: alice(),
    }
}

pub fn store_mock_contract(contract_id: u64, installments: u64) {
    store_mock_contract_with(contract_id, installments, |_| {})
}

pub fn store_mock_contract_with<F>(contract_id: u64, installments: u64, contract_fn: F)
where
    F: FnOnce(&mut Contract),
{
    let mut contract = mock_contract(contract_id, installments);
    contract_fn(&mut contract);

    ContractStorage::insert_contract(contract);
}

pub fn with_mock_contract<F>(id: u64, installments: u64, f: F) -> Contract
where
    F: FnOnce(&mut Contract),
{
    let mut contract = mock_contract(id, installments);
    f(&mut contract);
    contract
}

pub fn alice() -> Principal {
    Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()
}
