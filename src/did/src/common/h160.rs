use std::borrow::Cow;
use std::rc::Rc;

use candid::types::{Type, TypeInner};
use candid::{CandidType, Deserialize};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialOrd, Ord, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(transparent)]
pub struct H160(pub ethereum_types::H160);

impl CandidType for H160 {
    fn _ty() -> candid::types::Type {
        Type(Rc::new(TypeInner::Text))
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        serializer.serialize_text(&self.to_hex_str())
    }
}

impl From<ethereum_types::H160> for H160 {
    fn from(h160: ethereum_types::H160) -> Self {
        Self(h160)
    }
}

fn from_hex_str<const SIZE: usize>(mut s: &str) -> Result<[u8; SIZE], hex::FromHexError> {
    if s.starts_with("0x") || s.starts_with("0X") {
        s = &s[2..];
    }

    let mut result = [0u8; SIZE];
    hex::decode_to_slice(s, &mut result).and(Ok(result))
}

impl H160 {
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(ethereum_types::H160::from_slice(slice))
    }

    pub fn from_hex_str(s: &str) -> Result<Self, hex::FromHexError> {
        Ok(Self(ethereum_types::H160::from(from_hex_str::<20>(s)?)))
    }

    pub fn to_hex_str(&self) -> String {
        format!("0x{:x}", self.0)
    }

    pub const fn zero() -> Self {
        Self(ethereum_types::H160::zero())
    }
}

impl Storable for H160 {
    const BOUND: Bound = Bound::Bounded {
        max_size: 20,
        is_fixed_size: true,
    };

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        self.0.as_ref().into()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(ethereum_types::H160::from_slice(bytes.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use candid::{Decode, Encode};
    use ic_stable_structures::Storable;

    use super::*;

    fn generate_hex_str(size: usize) -> (Vec<u8>, String) {
        (0..size)
            .map(|i| {
                let val = (i * 13 % 255) as u8;
                (val, format!("{val:02x}"))
            })
            .unzip()
    }

    #[test]
    fn test_storable_h160() {
        let bytes: Vec<_> = (0..20).collect();
        let h160 = H160::from_slice(&bytes);

        let serialized = h160.to_bytes();
        let deserialized = H160::from_bytes(serialized);

        assert_eq!(h160, deserialized);
    }

    #[test]
    fn test_h160_from_str() {
        let (hex_val, str_val) = generate_hex_str(20);
        let value = H160::from_slice(&hex_val);
        assert_eq!(value, H160::from_hex_str(&str_val).unwrap());

        assert_eq!(value, H160::from_hex_str(&format!("0x{str_val}")).unwrap());
        assert_eq!(value, H160::from_hex_str(&format!("0X{str_val}")).unwrap());
        assert_eq!(
            value,
            H160::from_hex_str(&format!("0x{}", str_val.to_uppercase())).unwrap()
        );

        assert!(H160::from_hex_str("").is_err());
        assert!(H160::from_hex_str("01").is_err());
        assert!(H160::from_hex_str("012").is_err());
        assert!(H160::from_hex_str(&str_val.replace('0', "g")).is_err());
    }

    #[test]
    fn test_hex_from_str_returns_error() {
        let (_, str_val) = generate_hex_str(31);
        let val = H160::from_hex_str(&str_val);
        assert_eq!(val.unwrap_err(), hex::FromHexError::InvalidStringLength);

        let (_, str_val) = generate_hex_str(50);
        let val = H160::from_hex_str(&str_val);
        assert_eq!(val.unwrap_err(), hex::FromHexError::InvalidStringLength);
    }

    #[test]
    fn test_h160_from_address() {
        let (_, hex_string) = generate_hex_str(20);
        H160::from_hex_str(&hex_string).unwrap();
    }

    #[test]
    fn test_candid_type_h160() {
        let bytes: Vec<_> = (0..20).collect();
        let h160 = H160::from_slice(&bytes);

        let encoded = Encode!(&h160).unwrap();
        let decoded = Decode!(&encoded, H160).unwrap();

        assert_eq!(h160, decoded);
    }

    #[test]
    fn test_serde_h160() {
        let h160 = H160::from(ethereum_types::H160::random());

        let encoded = serde_json::json!(&h160);
        let decoded = serde_json::from_value(encoded).unwrap();

        assert_eq!(h160, decoded);
    }

    #[test]
    fn test_h160_fmt_lower_hex() {
        let value: H160 = ethereum_types::H160::random().into();
        let lower_hex = value.to_hex_str();
        assert!(lower_hex.starts_with("0x"));
        assert_eq!(value, H160::from_hex_str(&lower_hex).unwrap());
    }

    #[test]
    fn test_h160_transparent_serde_serialization() {
        let value: H160 = ethereum_types::H160::random().into();

        let encoded_value = serde_json::json!(&value);
        let decoded_primitive: ethereum_types::H160 =
            serde_json::from_value(encoded_value).unwrap();
        let encoded_primitive = serde_json::json!(&decoded_primitive);
        let decoded_value: H160 = serde_json::from_value(encoded_primitive).unwrap();

        assert_eq!(value, decoded_value);
    }
}
