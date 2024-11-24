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
    pub logo: Option<String>,
    pub mobile: String,
    pub name: String,
    pub owner: Principal,
    pub region: String,
    pub vat: String,
    pub website: String,
    pub zip_code: String,
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
