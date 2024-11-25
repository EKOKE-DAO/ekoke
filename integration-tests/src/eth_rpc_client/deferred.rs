use ethers_core::abi::{AbiDecode, AbiEncode};

use super::EthRpcClient;
use crate::evm::abi::{self, SellContract};
use crate::TestEnv;

pub struct DeferredErc721Client<'a> {
    env: &'a TestEnv,
}

impl<'a> DeferredErc721Client<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }
}

impl<'a> DeferredErc721Client<'a> {
    /// Get contract for token
    pub async fn token_contract(&self, token: u64) -> anyhow::Result<SellContract> {
        // make request
        let request = abi::deferred::TokenContractCall {
            token_id: token.into(),
        }
        .encode_hex();

        let client = EthRpcClient::from(self.env);
        let output = client.eth_call(self.env.evm.deferred, request).await?;
        let response = abi::deferred::TokenContractReturn::decode_hex(&output)?;

        Ok(response.0)
    }
}
