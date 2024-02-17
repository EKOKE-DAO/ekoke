use did::deferred::Contract;
use did::ID;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct GetContractRequest {
    pub id: ID,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetContractResponse {
    contract: Option<Contract>,
}

impl From<Option<Contract>> for GetContractResponse {
    fn from(contract: Option<Contract>) -> Self {
        Self { contract }
    }
}

pub type GetContractsResponse = Vec<ID>;
