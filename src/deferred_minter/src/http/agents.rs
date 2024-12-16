use std::str::FromStr as _;

use did::deferred::Agency;
use url::Url;

const FILTER_POSITION_LATITUDE: &str = "latitude";
const FILTER_POSITION_LONGITUDE: &str = "longitude";
const FILTER_POSITION_RADIUS: &str = "radius";

const FILTER_NAME: &str = "name";
const FILTER_ADDRESS: &str = "address";
const FILTER_COUNTRY: &str = "country";
const FILTER_CONTINENT: &str = "continent";
const FILTER_REGION: &str = "region";
const FILTER_ZIPCODE: &str = "zip_code";
const FILTER_VAT: &str = "vat";
const FILTER_CITY: &str = "city";

#[derive(Debug)]
/// Filter type to filter a contract
enum AgencyFilter {
    /// Always accept the request.
    Always,
    /// Has property with the given name and the value is contained in the property value.
    HasProperty { name: String, value: String },
    /// Position
    Position {
        latitude: f64,
        longitude: f64,
        radius: f64,
    },
}

impl AgencyFilter {
    fn agency_property(prop: &str, agency: &Agency) -> Option<String> {
        match prop {
            FILTER_NAME => Some(&agency.name).cloned(),
            FILTER_ADDRESS => Some(&agency.address).cloned(),
            FILTER_COUNTRY => Some(&agency.country).cloned(),
            FILTER_CONTINENT => Some(agency.continent.to_string()),
            FILTER_REGION => Some(&agency.region).cloned(),
            FILTER_ZIPCODE => Some(&agency.zip_code).cloned(),
            FILTER_VAT => Some(&agency.vat).cloned(),
            FILTER_CITY => Some(&agency.city).cloned(),
            _ => None,
        }
    }

    /// Check if the agency satisfies the filter.
    fn check(&self, agency: &Agency) -> bool {
        match self {
            AgencyFilter::Always => true,
            AgencyFilter::HasProperty { name, value } => Self::agency_property(name, agency)
                .map_or(false, |v| {
                    v.to_string().to_lowercase().contains(&value.to_lowercase())
                }),
            AgencyFilter::Position {
                latitude,
                longitude,
                radius,
            } => self.check_in_range(agency, *latitude, *longitude, *radius),
        }
    }

    /// Check if the contract property is in the given range.
    fn check_in_range(&self, agency: &Agency, latitude: f64, longitude: f64, radius: f64) -> bool {
        // get the position of the contract
        let Some(agency_latitude) = agency.lat.as_deref().and_then(|x| f64::from_str(x).ok())
        else {
            return false;
        };
        let Some(agency_longitude) = agency.lng.as_deref().and_then(|x| f64::from_str(x).ok())
        else {
            return false;
        };

        const EARTH_RADIUS_KM: f64 = 6371.0;

        // convert to radians
        let latitude = latitude.to_radians();
        let longitude = longitude.to_radians();
        let contract_latitude = agency_latitude.to_radians();
        let contract_longitude = agency_longitude.to_radians();

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

#[derive(Debug)]
pub struct Filters {
    filters: Vec<AgencyFilter>,
}

impl From<&Url> for Filters {
    fn from(url: &Url) -> Self {
        let mut filters = vec![AgencyFilter::Always];

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
                filters.push(AgencyFilter::Position {
                    latitude,
                    longitude,
                    radius,
                });
            }
        }

        for (name, value) in url.query_pairs() {
            match name.as_ref() {
                FILTER_NAME | FILTER_ADDRESS | FILTER_COUNTRY | FILTER_CONTINENT
                | FILTER_REGION | FILTER_ZIPCODE | FILTER_VAT | FILTER_CITY => {
                    let value = if value.is_empty() {
                        "true".to_string()
                    } else {
                        value.to_string()
                    };
                    filters.push(AgencyFilter::HasProperty {
                        name: name.to_string(),
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
    /// Check if the [`Agency`] satisfies the filters.
    pub fn check(&self, agency: &Agency) -> bool {
        println!("Filters checking {:?}", self.filters);
        self.filters.iter().all(|filter| filter.check(agency))
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::with_mock_agency;

    #[test]
    fn test_should_get_position_filter_from_url() {
        let url =
            Url::parse("http://example.com/?latitude=45.0&longitude=9.0&radius=10.0").unwrap();

        let filters = Filters::from(&url);
        let position = filters.filters.iter().find_map(|filter| match filter {
            AgencyFilter::Position {
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
        let agency = with_mock_agency(|agency| {
            agency.lat = Some("45.0".to_string());
            agency.lng = Some("9.0".to_string());
        });

        let filter = AgencyFilter::Position {
            latitude: 45.06,
            longitude: 9.08,
            radius: 10.0,
        };

        assert_eq!(filter.check(&agency), true);

        let filter = AgencyFilter::Position {
            latitude: 44.0,
            longitude: 8.0,
            radius: 1.0,
        };

        assert_eq!(filter.check(&agency), false);
    }
}
