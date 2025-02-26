use candid::Principal;
use did::deferred::{Continent, RealEstate};
use url::Url;

const FILTER_AGENT: &str = "agent";

const FILTER_POSITION_LATITUDE: &str = "latitude";
const FILTER_POSITION_LONGITUDE: &str = "longitude";
const FILTER_POSITION_RADIUS: &str = "radius";

const FILTER_PROPERTY_NAME: &str = "name";
const FILTER_PROPERTY_DESCRIPTION: &str = "description";
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

/// Filter type to filter a contract
enum RealEstateFilter {
    /// Always accept the request.
    Always,
    Name(String),
    Description(String),
    Address(String),
    Country(String),
    Continent(Continent),
    Region(String),
    ZipCode(String),
    Zone(String),
    City(String),
    SquareMeters(u64),
    Rooms(u64),
    Bathrooms(u64),
    Floors(u64),
    Balconies(u64),
    Garden,
    Pool,
    Garage,
    Parking,
    EnergyClass(String),
    /// Check if the agent is...
    Agent(Principal),
    /// Position
    Position {
        latitude: f64,
        longitude: f64,
        radius: f64,
    },
}

impl RealEstateFilter {
    /// Check if the contract satisfies the filter.
    fn check(&self, real_estate: &RealEstate) -> bool {
        match self {
            RealEstateFilter::Always => true,
            RealEstateFilter::Name(name) => real_estate.name == *name,
            RealEstateFilter::Description(description) => real_estate.description == *description,
            RealEstateFilter::Address(address) => real_estate.address == Some(address.clone()),
            RealEstateFilter::Country(country) => real_estate.country == Some(country.clone()),
            RealEstateFilter::Continent(continent) => real_estate.continent == Some(*continent),
            RealEstateFilter::Region(region) => real_estate.region == Some(region.clone()),
            RealEstateFilter::ZipCode(zip_code) => real_estate.zip_code == Some(zip_code.clone()),
            RealEstateFilter::Zone(zone) => real_estate.zone == Some(zone.clone()),
            RealEstateFilter::City(city) => real_estate.city == Some(city.clone()),
            RealEstateFilter::SquareMeters(square_meters) => {
                real_estate.square_meters == Some(*square_meters)
            }
            RealEstateFilter::Rooms(rooms) => real_estate.rooms == Some(*rooms),
            RealEstateFilter::Bathrooms(bathrooms) => real_estate.bathrooms == Some(*bathrooms),
            RealEstateFilter::Floors(floors) => real_estate.floors == Some(*floors),
            RealEstateFilter::Balconies(balconies) => real_estate.balconies == Some(*balconies),
            RealEstateFilter::Garden => real_estate.garden.unwrap_or_default(),
            RealEstateFilter::Pool => real_estate.pool.unwrap_or_default(),
            RealEstateFilter::Garage => real_estate.garage.unwrap_or_default(),
            RealEstateFilter::Parking => real_estate.parking.unwrap_or_default(),
            RealEstateFilter::EnergyClass(energy_class) => {
                real_estate.energy_class == Some(energy_class.clone())
            }

            RealEstateFilter::Agent(agent) => real_estate.agency == *agent,
            RealEstateFilter::Position {
                latitude,
                longitude,
                radius,
            } => self.check_in_range(real_estate, *latitude, *longitude, *radius),
        }
    }

    /// Check if the contract property is in the given range.
    fn check_in_range(
        &self,
        real_estate: &RealEstate,
        latitude: f64,
        longitude: f64,
        radius: f64,
    ) -> bool {
        // get the position of the contract
        let Some(contract_latitude) = real_estate.latitude else {
            return false;
        };
        let Some(contract_longitude) = real_estate.longitude else {
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
}

pub struct RealEstateFilters {
    filters: Vec<RealEstateFilter>,
}

impl From<&Url> for RealEstateFilters {
    fn from(url: &Url) -> Self {
        let mut filters = vec![RealEstateFilter::Always];

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
                filters.push(RealEstateFilter::Position {
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
                        filters.push(RealEstateFilter::Agent(agent));
                    }
                }
                FILTER_PROPERTY_NAME => filters.push(RealEstateFilter::Name(value.to_string())),
                FILTER_PROPERTY_DESCRIPTION => {
                    filters.push(RealEstateFilter::Description(value.to_string()))
                }
                FILTER_PROPERTY_ADDRESS => {
                    filters.push(RealEstateFilter::Address(value.to_string()))
                }
                FILTER_PROPERTY_COUNTRY => {
                    filters.push(RealEstateFilter::Country(value.to_string()))
                }
                FILTER_PROPERTY_CONTINENT => {
                    if let Ok(continent) = value.parse() {
                        filters.push(RealEstateFilter::Continent(continent));
                    }
                }
                FILTER_PROPERTY_REGION => filters.push(RealEstateFilter::Region(value.to_string())),
                FILTER_PROPERTY_ZIPCODE => {
                    filters.push(RealEstateFilter::ZipCode(value.to_string()))
                }
                FILTER_PROPERTY_ZONE => filters.push(RealEstateFilter::Zone(value.to_string())),
                FILTER_PROPERTY_CITY => filters.push(RealEstateFilter::City(value.to_string())),
                FILTER_PROPERTY_SQUAREMETERS => {
                    if let Ok(square_meters) = value.parse() {
                        filters.push(RealEstateFilter::SquareMeters(square_meters));
                    }
                }
                FILTER_PROPERTY_ROOMS => {
                    if let Ok(rooms) = value.parse() {
                        filters.push(RealEstateFilter::Rooms(rooms));
                    }
                }
                FILTER_PROPERTY_BATHROOMS => {
                    if let Ok(bathrooms) = value.parse() {
                        filters.push(RealEstateFilter::Bathrooms(bathrooms));
                    }
                }
                FILTER_PROPERTY_FLOORS => {
                    if let Ok(floors) = value.parse() {
                        filters.push(RealEstateFilter::Floors(floors));
                    }
                }
                FILTER_PROPERTY_BALCONIES => {
                    if let Ok(balconies) = value.parse() {
                        filters.push(RealEstateFilter::Balconies(balconies));
                    }
                }
                FILTER_PROPERTY_GARDEN => {
                    filters.push(RealEstateFilter::Garden);
                }
                FILTER_PROPERTY_POOL => {
                    filters.push(RealEstateFilter::Pool);
                }
                FILTER_PROPERTY_GARAGE => {
                    filters.push(RealEstateFilter::Garage);
                }
                FILTER_PROPERTY_PARKING => {
                    filters.push(RealEstateFilter::Parking);
                }
                FILTER_PROPERTY_ENERGYCLASS => {
                    filters.push(RealEstateFilter::EnergyClass(value.to_string()))
                }
                _ => {}
            }
        }

        RealEstateFilters { filters }
    }
}

impl RealEstateFilters {
    /// Check if the contract satisfies the filters.
    pub fn check(&self, real_estate: &RealEstate) -> bool {
        self.filters.iter().all(|filter| filter.check(real_estate))
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_get_position_filter_from_url() {
        let url =
            Url::parse("http://example.com/?latitude=45.0&longitude=9.0&radius=10.0").unwrap();

        let filters = RealEstateFilters::from(&url);
        let position = filters.filters.iter().find_map(|filter| match filter {
            RealEstateFilter::Position {
                latitude,
                longitude,
                radius,
            } => Some((*latitude, *longitude, *radius)),
            _ => None,
        });

        assert_eq!(position, Some((45.0, 9.0, 10.0)));
    }
}
