use candid::Principal;
use did::deferred::{
    Agency, Continent, Contract, ContractDocument, ContractType, GenericValue, RestrictedProperty,
    RestrictionLevel,
};

fn main() -> anyhow::Result<()> {
    repr_contract()?;

    Ok(())
}

fn repr_contract() -> anyhow::Result<()> {
    let agency = Agency {
        address: "Via Roma 1".to_string(),
        name: "Agency".to_string(),
        agent: "Miriam".to_string(),
        city: "Milano".to_string(),
        continent: Continent::Europe,
        country: "Italy".to_string(),
        email: "miriamlagente@gmail.com".to_string(),
        logo: Some("logo.png".to_string()),
        mobile: "+39 333 1234567".to_string(),
        owner: Principal::from_text("v5vof-zqaaa-aaaal-ai5cq-cai")?,
        region: "Lombardia".to_string(),
        vat: "IT12345678901".to_string(),
        website: "https://www.agency.com".to_string(),
        zip_code: "20121".to_string(),
    };

    let properties = vec![
        (
            "contract:address".to_string(),
            GenericValue::TextContent("Via Milano 12".to_string()),
        ),
        (
            "contract:city".to_string(),
            GenericValue::TextContent("Milano".to_string()),
        ),
        (
            "contract:zip_code".to_string(),
            GenericValue::TextContent("20121".to_string()),
        ),
        (
            "contract:country".to_string(),
            GenericValue::TextContent("Italy".to_string()),
        ),
        (
            "contract:continent".to_string(),
            GenericValue::TextContent("Europe".to_string()),
        ),
        (
            "contract:region".to_string(),
            GenericValue::TextContent("Lombardia".to_string()),
        ),
        (
            "contract:deposit".to_string(),
            GenericValue::Nat64Content(1000),
        ),
        (
            "contract:installments".to_string(),
            GenericValue::Nat64Content(1000),
        ),
        (
            "contract:expiration".to_string(),
            GenericValue::TextContent("1970-01-01".to_string()),
        ),
        (
            "contract:currency".to_string(),
            GenericValue::TextContent("EUR".to_string()),
        ),
        (
            "contract:value".to_string(),
            GenericValue::Nat64Content(100000),
        ),
    ];

    let restricted_properties = vec![(
        "contract:owner".to_string(),
        RestrictedProperty {
            access_list: vec![RestrictionLevel::Agent, RestrictionLevel::Seller],
            value: GenericValue::TextContent("Miriam".to_string()),
        },
    )];

    let documents = vec![(
        1u64,
        ContractDocument {
            access_list: vec![RestrictionLevel::Agent, RestrictionLevel::Seller],
            mime_type: "application/pdf".to_string(),
        },
    )];

    let contract = Contract {
        r#type: ContractType::Sell,
        sellers: Vec::new(),
        buyers: Vec::new(),
        value: 0,
        currency: String::new(),
        deposit: 0,
        installments: 1000,
        expiration: "1970-01-01".to_string(),
        properties,
        restricted_properties,
        agency: Some(agency),
        id: 1u64.into(),
        documents,
        closed: false,
    };

    let encoded = serde_json::to_string_pretty(&contract)?;

    println!("Encoded contract");
    println!("{encoded}",);
    println!("---------------------------------");

    Ok(())
}
