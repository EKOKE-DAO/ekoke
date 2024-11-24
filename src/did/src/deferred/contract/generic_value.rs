use candid::{CandidType, Deserialize, Int, Nat, Principal};
use serde::Serialize;

/// Properties value representation for a token
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum GenericValue {
    BoolContent(bool),
    TextContent(String),
    BlobContent(Vec<u8>),
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
    NestedContent(Vec<(String, GenericValue)>),
}
