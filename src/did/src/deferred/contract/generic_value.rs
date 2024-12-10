use std::fmt;

use candid::{CandidType, Deserialize, Int, Nat, Principal};
use serde::Serialize;

/// Properties value representation for a token
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum GenericValue {
    BoolContent(bool),
    TextContent(String),
    Principal(Principal),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    NatContent(Nat),
    Int8Content(i8),
    Int16Content(i16),
    Int32Content(i32),
    Int64Content(i64),
    IntContent(Int),
    FloatContent(f64), // motoko only support f64
}

impl GenericValue {
    /// Check if the value equals to the given string
    pub fn equals_str(&self, s: &str) -> bool {
        self.to_string() == s
    }
}

impl fmt::Display for GenericValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&match self {
            GenericValue::BoolContent(v) => v.to_string(),
            GenericValue::TextContent(v) => v.clone(),
            GenericValue::Principal(v) => v.to_text(),
            GenericValue::Nat8Content(v) => v.to_string(),
            GenericValue::Nat16Content(v) => v.to_string(),
            GenericValue::Nat32Content(v) => v.to_string(),
            GenericValue::Nat64Content(v) => v.to_string(),
            GenericValue::NatContent(v) => v.to_string(),
            GenericValue::Int8Content(v) => v.to_string(),
            GenericValue::Int16Content(v) => v.to_string(),
            GenericValue::Int32Content(v) => v.to_string(),
            GenericValue::Int64Content(v) => v.to_string(),
            GenericValue::IntContent(v) => v.to_string(),
            GenericValue::FloatContent(v) => v.to_string(),
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_generic_value_to_string() {
        let v = GenericValue::BoolContent(true);
        assert_eq!(v.to_string(), "true");

        let v = GenericValue::TextContent("hello".to_string());
        assert_eq!(v.to_string(), "hello");

        let v = GenericValue::Principal(
            Principal::from_text("v5vof-zqaaa-aaaal-ai5cq-cai").expect("invalid principal"),
        );
        assert_eq!(v.to_string(), "v5vof-zqaaa-aaaal-ai5cq-cai");

        let v = GenericValue::Nat8Content(8);
        assert_eq!(v.to_string(), "8");

        let v = GenericValue::Nat16Content(16);
        assert_eq!(v.to_string(), "16");

        let v = GenericValue::Nat32Content(32);
        assert_eq!(v.to_string(), "32");

        let v = GenericValue::Nat64Content(64);
        assert_eq!(v.to_string(), "64");

        let v = GenericValue::NatContent(Nat::from(128u64));
        assert_eq!(v.to_string(), "128");

        let v = GenericValue::Int8Content(-8);
        assert_eq!(v.to_string(), "-8");

        let v = GenericValue::Int16Content(-16);
        assert_eq!(v.to_string(), "-16");

        let v = GenericValue::Int32Content(-32);
        assert_eq!(v.to_string(), "-32");

        let v = GenericValue::Int64Content(-64);
        assert_eq!(v.to_string(), "-64");

        let v = GenericValue::IntContent(Int::from(-128));
        assert_eq!(v.to_string(), "-128");

        let v = GenericValue::FloatContent(3.14);
        assert_eq!(v.to_string(), "3.14");
    }

    #[test]
    fn test_eq_str() {
        let v = GenericValue::BoolContent(true);
        assert_eq!(v.equals_str("true"), true);
        assert_eq!(v.equals_str("false"), false);
    }
}
