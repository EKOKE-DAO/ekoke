use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use dip721::TokenIdentifier;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::Serialize;

use crate::ekoke::PicoEkoke;
use crate::ID;

/// A Non fungible token related to an installment of a contract
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Token {
    /// Unique identifier of the token
    pub id: TokenIdentifier,
    /// Contract id
    pub contract_id: ID,
    /// Token owner. If none the token is burned
    pub owner: Option<Principal>,
    /// Value of the single token (FIAT)
    pub value: u64,
    /// $picoEkoke (pico-ekoke) reward for buying a Token
    pub picoekoke_reward: PicoEkoke,
    /// A principal who can operate on the token
    pub operator: Option<Principal>,
    /// Whether the token is burned
    pub is_burned: bool,
    /// Timestamp the token was minted at
    pub minted_at: u64,
    /// Principal who minted the token
    pub minted_by: Principal,
    /// Timestamp the token was approved at
    pub approved_at: Option<u64>,
    /// Principal who approved the token
    pub approved_by: Option<Principal>,
    /// Timestamp the token was burned at
    pub burned_at: Option<u64>,
    /// Principal who burned the token
    pub burned_by: Option<Principal>,
    /// Timestamp the token was transferred at
    pub transferred_at: Option<u64>,
    /// Principal who transferred the token
    pub transferred_by: Option<Principal>,
}

impl Storable for Token {
    const BOUND: Bound = Bound::Bounded {
        max_size: 512,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}
