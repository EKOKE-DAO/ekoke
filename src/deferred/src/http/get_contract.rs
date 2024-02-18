use did::ID;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct GetContractRequest {
    pub id: ID,
}
