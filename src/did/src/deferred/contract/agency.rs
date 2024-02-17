use candid::{CandidType, Decode, Encode};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};

/// A sell contract for a building
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub struct Agency {
    pub name: String,
    pub address: String,
    pub city: String,
    pub region: String,
    pub zip_code: String,
    pub country: String,
    pub continent: Continent,
    pub email: String,
    pub website: String,
    pub mobile: String,
    pub vat: String,
    pub agent: String,
    pub logo: Option<String>,
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

impl Storable for Agency {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}
