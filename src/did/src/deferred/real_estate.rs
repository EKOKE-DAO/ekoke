use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::Serialize;

use super::{AgencyId, Continent};
use crate::ID;

/// Data for a real estate
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub struct RealEstate {
    /// Unique identifier for the real estate
    pub id: ID,
    /// agency
    pub agency: AgencyId,
    /// name
    pub name: String,
    /// description
    pub description: String,
    /// image URL
    pub image: Option<String>,
    /// address
    pub address: Option<String>,
    /// country
    pub country: Option<String>,
    /// continent
    pub continent: Option<Continent>,
    /// region
    pub region: Option<String>,
    /// city
    pub city: Option<String>,
    /// zone
    pub zone: Option<String>,
    /// postal code
    pub zip_code: Option<String>,
    /// latitude
    pub latitude: Option<f64>,
    /// longitude
    pub longitude: Option<f64>,
    /// square meters
    pub square_meters: Option<u64>,
    /// number of rooms
    pub rooms: Option<u64>,
    /// number of bathrooms
    pub bathrooms: Option<u64>,
    /// number of bedrooms
    pub bedrooms: Option<u64>,
    /// floors
    pub floors: Option<u64>,
    /// year of construction
    pub year_of_construction: Option<u64>,
    /// garden
    pub garden: Option<bool>,
    /// balconies
    pub balconies: Option<u64>,
    /// pool
    pub pool: Option<bool>,
    /// garage
    pub garage: Option<bool>,
    /// parking
    pub parking: Option<bool>,
    /// elevator
    pub elevator: Option<bool>,
    /// energy class
    pub energy_class: Option<String>,
    /// youtube url
    pub youtube: Option<String>,
}

impl Storable for RealEstate {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

#[cfg(test)]
mod test {

    use candid::Principal;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_and_decode_real_estate() {
        let real_estate = RealEstate {
            id: 2_u64.into(),
            agency: Principal::management_canister(),
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
        };

        let data = Encode!(&real_estate).unwrap();
        let decoded = Decode!(&data, RealEstate).unwrap();

        assert_eq!(real_estate, decoded);
    }
}
