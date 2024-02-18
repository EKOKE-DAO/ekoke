use did::deferred::TokenIdentifier;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct GetTokenRequest {
    pub id: TokenIdentifier,
}
