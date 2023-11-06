//! # Common
//!
//! Common types

use std::fmt::Display;
use std::rc::Rc;

use candid::types::{Type, TypeInner};
use candid::{CandidType, Decode, Deserialize, Encode};
use ethereum_types::H160;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;

/// An identifier in the dilazionato environment. It has the same syntax as an Ethereum address
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ID(H160);

impl From<H160> for ID {
    fn from(value: H160) -> Self {
        Self(value)
    }
}

impl ID {
    /// Generate a random ID
    pub fn random() -> Self {
        H160::random().into()
    }

    /// Get the ID as a H160
    pub fn as_h160(&self) -> &H160 {
        &self.0
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        Self(ethereum_types::H160::from_slice(slice))
    }

    pub fn as_hex_str(&self) -> String {
        format!("0x{:x}", self.0)
    }

    pub const fn zero() -> Self {
        Self(ethereum_types::H160::zero())
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_hex_str())
    }
}

impl CandidType for ID {
    fn _ty() -> candid::types::Type {
        Type(Rc::new(TypeInner::Text))
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        serializer.serialize_text(&self.as_hex_str())
    }
}

impl Storable for ID {
    const BOUND: Bound = Bound::Bounded {
        max_size: 30,
        is_fixed_size: true,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self.as_h160().as_bytes()).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let bytes: [u8; 20] = Decode!(&bytes, [u8; 20]).unwrap();
        let h160 = H160::from_slice(&bytes);

        Self::from(h160)
    }
}
