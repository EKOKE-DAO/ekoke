use std::fmt;

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};

/// A sell contract for a building
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub struct Agency {
    pub address: String,
    pub agent: String,
    pub city: String,
    pub continent: Continent,
    pub country: String,
    pub email: String,
    pub lat: Option<String>,
    pub lng: Option<String>,
    pub logo: Option<String>,
    pub mobile: String,
    pub name: String,
    pub owner: Principal,
    pub region: String,
    pub vat: String,
    pub website: String,
    pub zip_code: String,
}

impl Default for Agency {
    fn default() -> Self {
        Self {
            region: Default::default(),
            vat: Default::default(),
            website: Default::default(),
            zip_code: Default::default(),
            address: Default::default(),
            agent: Default::default(),
            city: Default::default(),
            country: Default::default(),
            email: Default::default(),
            logo: Default::default(),
            lat: Default::default(),
            lng: Default::default(),
            mobile: Default::default(),
            name: Default::default(),
            owner: Principal::anonymous(),
            continent: Continent::Europe,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, Copy, PartialEq, Eq)]
pub enum Continent {
    Africa,
    Antarctica,
    Asia,
    Europe,
    NorthAmerica,
    Oceania,
    SouthAmerica,
}

impl fmt::Display for Continent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Continent::Africa => "Africa",
            Continent::Antarctica => "Antarctica",
            Continent::Asia => "Asia",
            Continent::Europe => "Europe",
            Continent::NorthAmerica => "North America",
            Continent::Oceania => "Oceania",
            Continent::SouthAmerica => "South America",
        };
        write!(f, "{s}",)
    }
}

impl Storable for Agency {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}
