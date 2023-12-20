use candid::CandidType;
use serde::Deserialize;

use super::{Contract, Token};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TokenInfo {
    pub token: Token,
    pub contract: Contract,
}
