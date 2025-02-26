use candid::Principal;
use did::deferred::Contract;
use did::H160;
use url::Url;

const FILTER_SELLER: &str = "seller";
const FILTER_BUYER: &str = "buyer";
const FILTER_AGENT: &str = "agent";

const FILTER_MIN_PRICE: &str = "minPrice";
const FILTER_MAX_PRICE: &str = "maxPrice";

const FILTER_POSITION_LATITUDE: &str = "latitude";
const FILTER_POSITION_LONGITUDE: &str = "longitude";
const FILTER_POSITION_RADIUS: &str = "radius";

const FILTER_PROPERTY_NAME: &str = "name";
const FILTER_PROPERTY_DESCRIPTION: &str = "description";
const FILTER_PROPERTY_IMAGE: &str = "image";
const FILTER_PROPERTY_ADDRESS: &str = "address";
const FILTER_PROPERTY_COUNTRY: &str = "country";
const FILTER_PROPERTY_CONTINENT: &str = "continent";
const FILTER_PROPERTY_REGION: &str = "region";
const FILTER_PROPERTY_ZIPCODE: &str = "zipCode";
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

const CONTRACT_LATITUDE: &str = "contract:latitude";
const CONTRACT_LONGITUDE: &str = "contract:longitude";

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
    /// Min price
    MinPrice(u64),
    /// Max price
    MaxPrice(u64),
    /// Position
    Position {
        latitude: f64,
        longitude: f64,
        radius: f64,
    },
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
                .is_some_and(|(_, v)| v.to_string().to_lowercase().contains(&value.to_lowercase())),
            ContractFilter::Seller(addr) => contract
                .sellers
                .iter()
                .any(|seller| seller.address == *addr),
            ContractFilter::Buyer(addr) => contract.buyers.iter().any(|buyer| buyer == addr),
            ContractFilter::Agent(agent) => contract.agency == *agent,
            ContractFilter::MinPrice(min_price) => contract.value >= *min_price,
            ContractFilter::MaxPrice(max_price) => contract.value <= *max_price,
            ContractFilter::Position {
                latitude,
                longitude,
                radius,
            } => self.check_in_range(contract, *latitude, *longitude, *radius),
        }
    }

    /// Check if the contract property is in the given range.
    fn check_in_range(
        &self,
        contract: &Contract,
        latitude: f64,
        longitude: f64,
        radius: f64,
    ) -> bool {
        // get the position of the contract
        let Some(contract_latitude) =
            Self::get_contract_property_as::<f64>(contract, CONTRACT_LATITUDE)
        else {
            return false;
        };
        let Some(contract_longitude) =
            Self::get_contract_property_as::<f64>(contract, CONTRACT_LONGITUDE)
        else {
            return false;
        };

        const EARTH_RADIUS_KM: f64 = 6371.0;

        // convert to radians
        let latitude = latitude.to_radians();
        let longitude = longitude.to_radians();
        let contract_latitude = contract_latitude.to_radians();
        let contract_longitude = contract_longitude.to_radians();

        // calculate the distance
        let delta_latitude = contract_latitude - latitude;
        let delta_longitude = contract_longitude - longitude;

        // haversine formula
        let a = (delta_latitude / 2.0).sin().powi(2)
            + latitude.cos() * contract_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        // distance in km
        let distance = EARTH_RADIUS_KM * c;

        distance <= radius
    }

    /// Get the property value of the contract as the given type.
    fn get_contract_property_as<T>(contract: &Contract, property: &str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        contract
            .properties
            .iter()
            .find(|(k, _)| k == property)
            .and_then(|(_, v)| v.to_string().parse().ok())
    }
}

pub struct ContractFilters {
    filters: Vec<ContractFilter>,
}

impl From<&Url> for ContractFilters {
    fn from(url: &Url) -> Self {
        let mut filters = vec![ContractFilter::Always];

        // check if there is position (search for latitude, longitude and radius)
        if let (Some(latitude), Some(longitude), Some(radius)) = (
            url.query_pairs()
                .find(|(name, _)| name == FILTER_POSITION_LATITUDE),
            url.query_pairs()
                .find(|(name, _)| name == FILTER_POSITION_LONGITUDE),
            url.query_pairs()
                .find(|(name, _)| name == FILTER_POSITION_RADIUS),
        ) {
            if let (Ok(latitude), Ok(longitude), Ok(radius)) =
                (latitude.1.parse(), longitude.1.parse(), radius.1.parse())
            {
                filters.push(ContractFilter::Position {
                    latitude,
                    longitude,
                    radius,
                });
            }
        }

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
                FILTER_MIN_PRICE => {
                    if let Ok(min_price) = value.parse() {
                        filters.push(ContractFilter::MinPrice(min_price));
                    }
                }
                FILTER_MAX_PRICE => {
                    if let Ok(max_price) = value.parse() {
                        filters.push(ContractFilter::MaxPrice(max_price));
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

        ContractFilters { filters }
    }
}

impl ContractFilters {
    /// Check if the contract satisfies the filters.
    pub fn check(&self, contract: &Contract) -> bool {
        self.filters.iter().all(|filter| filter.check(contract))
    }
}

#[cfg(test)]
mod test {

    use did::deferred::GenericValue;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::with_mock_contract;

    #[test]
    fn test_should_get_position_filter_from_url() {
        let url =
            Url::parse("http://example.com/?latitude=45.0&longitude=9.0&radius=10.0").unwrap();

        let filters = ContractFilters::from(&url);
        let position = filters.filters.iter().find_map(|filter| match filter {
            ContractFilter::Position {
                latitude,
                longitude,
                radius,
            } => Some((*latitude, *longitude, *radius)),
            _ => None,
        });

        assert_eq!(position, Some((45.0, 9.0, 10.0)));
    }

    #[test]
    fn test_should_check_in_position() {
        let contract = with_mock_contract(1, 100, |contract| {
            contract.properties.push((
                "contract:latitude".to_string(),
                GenericValue::TextContent("45.0".to_string()),
            ));
            contract.properties.push((
                "contract:longitude".to_string(),
                GenericValue::TextContent("9.0".to_string()),
            ));
        });

        let filter = ContractFilter::Position {
            latitude: 45.06,
            longitude: 9.08,
            radius: 10.0,
        };

        assert_eq!(filter.check(&contract), true);

        let filter = ContractFilter::Position {
            latitude: 44.0,
            longitude: 8.0,
            radius: 1.0,
        };

        assert_eq!(filter.check(&contract), false);
    }
}
