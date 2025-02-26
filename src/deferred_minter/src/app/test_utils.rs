use candid::Principal;
use did::deferred::{
    Agency, Continent, Contract, GenericValue, RealEstate, RestrictedProperty, RestrictionLevel,
    Seller,
};
use did::H160;

use crate::utils::caller;

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
        real_estate: 1u64.into(),
        agency: mock_agency().owner,
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
        mobile: "mobile".to_string(),
        vat: "vat".to_string(),
        lat: None,
        lng: None,
        agent: "agent".to_string(),
        logo: None,
        owner: alice(),
    }
}

pub fn mock_real_estate() -> RealEstate {
    RealEstate {
        deleted: false,
        agency: caller(),
        name: "name".to_string(),
        description: "description".to_string(),
        image: Some("image".to_string()),
        address: Some("address".to_string()),
        country: Some("country".to_string()),
        continent: Some(Continent::Europe),
        region: Some("region".to_string()),
        city: Some("city".to_string()),
        zone: Some("zone".to_string()),
        zip_code: Some("zip_code".to_string()),
        latitude: Some(1.0),
        longitude: Some(2.0),
        square_meters: Some(100),
        rooms: Some(3),
        bathrooms: Some(2),
        bedrooms: Some(1),
        floors: Some(1),
        year_of_construction: Some(2021),
        garden: Some(true),
        balconies: Some(1),
        pool: Some(true),
        garage: Some(true),
        parking: Some(true),
        elevator: Some(true),
        energy_class: Some("A".to_string()),
        youtube: Some("youtube".to_string()),
    }
}

pub fn with_mock_agency<F>(f: F) -> Agency
where
    F: FnOnce(&mut Agency),
{
    let mut agency = mock_agency();
    f(&mut agency);

    agency
}

pub fn alice() -> Principal {
    Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()
}

pub fn bob() -> Principal {
    Principal::from_text("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe").unwrap()
}

pub fn charlie() -> Principal {
    Principal::from_text("vuwfz-pyaaa-aaaal-ai5da-cai").unwrap()
}
