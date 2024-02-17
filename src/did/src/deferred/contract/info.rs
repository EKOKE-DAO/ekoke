use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::{Contract, Token};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TokenInfo {
    pub token: Token,
    pub contract: Contract,
}
