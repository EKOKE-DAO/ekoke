use candid::Principal;
use did::deferred::Contract;
use did::H160;
use url::Url;

const FILTER_SELLER: &str = "seller";
const FILTER_BUYER: &str = "buyer";
const FILTER_AGENT: &str = "agent";

const FILTER_PROPERTY_NAME: &str = "name";
const FILTER_PROPERTY_DESCRIPTION: &str = "description";
const FILTER_PROPERTY_IMAGE: &str = "image";
const FILTER_PROPERTY_ADDRESS: &str = "address";
const FILTER_PROPERTY_COUNTRY: &str = "country";
const FILTER_PROPERTY_CONTINENT: &str = "continent";
const FILTER_PROPERTY_REGION: &str = "region";
const FILTER_PROPERTY_ZIPCODE: &str = "zipCode";
const FILTER_PROPERTY_LATITUDE: &str = "latitude";
const FILTER_PROPERTY_LONGITUDE: &str = "longitude";
const FILTER_PROPERTY_ZONE: &str = "zone";
const FILTER_PROPERTY_CITY: &str = "city";
const FILTER_PROPERTY_SQUAREMETERS: &str = "squareMeters";
const FILTER_PROPERTY_ROOMS: &str = "rooms";
const FILTER_PROPERTY_BATHROOMS: &str = "bathrooms";
const FILTER_PROPERTY_FLOORS: &str = "floors";
const FILTER_PROPERTY_BALCONIES: &str = "balconies";
const FILTER_PROPERTY_GARDEN: &str = "garden";
const FILTER_PROPERTY_POOL: &str = "pool";
const FILTER_PROPERTY_GARAGE: &str = "garage";
const FILTER_PROPERTY_PARKING: &str = "parking";
const FILTER_PROPERTY_ENERGYCLASS: &str = "energyClass";
const FILTER_PROPERTY_YOUTUBEURL: &str = "youtubeUrl";

/// Filter type to filter a contract
enum ContractFilter {
    /// Always accept the request.
    Always,
    /// Has property with the given name and the value is contained in the property value.
    HasProperty { name: String, value: String },
    /// Seller
    Seller(H160),
    /// Buyer
    Buyer(H160),
    /// Agent
    Agent(Principal),
}

impl ContractFilter {
    /// Check if the contract satisfies the filter.
    fn check(&self, contract: &Contract) -> bool {
        match self {
            ContractFilter::Always => true,
            ContractFilter::HasProperty { name, value } => contract
                .properties
                .iter()
                .find(|(k, _)| k == name)
                .map_or(false, |(_, v)| {
                    v.to_string().to_lowercase().contains(&value.to_lowercase())
                }),
            ContractFilter::Seller(addr) => contract
                .sellers
                .iter()
                .any(|seller| seller.address == *addr),
            ContractFilter::Buyer(addr) => contract.buyers.iter().any(|buyer| buyer == addr),
            ContractFilter::Agent(agent) => contract
                .agency
                .as_ref()
                .map(|agency| agency.owner == *agent)
                .unwrap_or_default(),
        }
    }
}

pub struct Filters {
    filters: Vec<ContractFilter>,
}

impl From<&Url> for Filters {
    fn from(url: &Url) -> Self {
        let mut filters = vec![ContractFilter::Always];

        for (name, value) in url.query_pairs() {
            match name.as_ref() {
                FILTER_AGENT => {
                    if let Ok(agent) = Principal::from_text(value) {
                        filters.push(ContractFilter::Agent(agent));
                    }
                }
                FILTER_BUYER => {
                    if let Ok(addr) = H160::from_hex_str(value.as_ref()) {
                        filters.push(ContractFilter::Buyer(addr));
                    }
                }
                FILTER_SELLER => {
                    if let Ok(addr) = H160::from_hex_str(value.as_ref()) {
                        filters.push(ContractFilter::Seller(addr));
                    }
                }
                FILTER_PROPERTY_NAME
                | FILTER_PROPERTY_DESCRIPTION
                | FILTER_PROPERTY_IMAGE
                | FILTER_PROPERTY_ADDRESS
                | FILTER_PROPERTY_COUNTRY
                | FILTER_PROPERTY_CONTINENT
                | FILTER_PROPERTY_REGION
                | FILTER_PROPERTY_ZIPCODE
                | FILTER_PROPERTY_LATITUDE
                | FILTER_PROPERTY_LONGITUDE
                | FILTER_PROPERTY_ZONE
                | FILTER_PROPERTY_CITY
                | FILTER_PROPERTY_SQUAREMETERS
                | FILTER_PROPERTY_ROOMS
                | FILTER_PROPERTY_BATHROOMS
                | FILTER_PROPERTY_FLOORS
                | FILTER_PROPERTY_BALCONIES
                | FILTER_PROPERTY_GARDEN
                | FILTER_PROPERTY_POOL
                | FILTER_PROPERTY_GARAGE
                | FILTER_PROPERTY_PARKING
                | FILTER_PROPERTY_ENERGYCLASS
                | FILTER_PROPERTY_YOUTUBEURL => {
                    let value = if value.is_empty() {
                        "true".to_string()
                    } else {
                        value.to_string()
                    };
                    filters.push(ContractFilter::HasProperty {
                        name: format!("contract:{name}"),
                        value,
                    });
                }
                _ => {}
            }
        }

        Filters { filters }
    }
}

impl Filters {
    /// Check if the contract satisfies the filters.
    pub fn check(&self, contract: &Contract) -> bool {
        self.filters.iter().all(|filter| filter.check(contract))
    }
}
