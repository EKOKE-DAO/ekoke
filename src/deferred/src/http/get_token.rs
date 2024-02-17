use did::deferred::{TokenIdentifier, TokenInfo};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct GetTokenRequest {
    pub id: TokenIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTokenResponse {
    token: Option<TokenInfo>,
}

impl From<Option<TokenInfo>> for GetTokenResponse {
    fn from(token: Option<TokenInfo>) -> Self {
        Self { token }
    }
}
