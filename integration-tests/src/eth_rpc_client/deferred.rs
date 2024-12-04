use did::H160;
use ethers_core::abi::{AbiDecode, AbiEncode};

use super::EthRpcClient;
use crate::evm::abi::{self, SellContract};
use crate::{TestEnv, WalletName};

pub struct DeferredErc721Client<'a, T>
where
    T: TestEnv,
{
    env: &'a T,
}

impl<'a, T> DeferredErc721Client<'a, T>
where
    T: TestEnv,
{
    pub fn new(env: &'a T) -> Self {
        Self { env }
    }

    /// Get contract for token
    pub async fn token_contract(&self, token: u64) -> anyhow::Result<SellContract> {
        // make request
        let request = abi::deferred::TokenContractCall {
            token_id: token.into(),
        }
        .encode_hex();

        let client = EthRpcClient::new(self.env);
        let output = client.eth_call(self.env.evm().deferred, request).await?;
        let response = abi::deferred::TokenContractReturn::decode_hex(&output)?;

        Ok(response.0)
    }

    pub async fn admin_set_minter(&self, minter: H160) -> anyhow::Result<()> {
        let request = abi::deferred::AdminSetDeferredMinterCall {
            deferred_minter: minter.0,
        }
        .encode();

        let client = EthRpcClient::new(self.env);
        client
            .send_raw_transaction(WalletName::Owner, self.env.evm().deferred, request.into())
            .await
    }

    pub async fn get_minter_address(&self) -> anyhow::Result<H160> {
        let request = abi::deferred::DeferredMinterCall {}.encode_hex();

        let client = EthRpcClient::new(self.env);
        let output = client.eth_call(self.env.evm().deferred, request).await?;

        let response = abi::deferred::DeferredMinterReturn::decode_hex(&output)?;

        Ok(response.0.into())
    }

    pub async fn admin_set_reward_pool(&self) -> anyhow::Result<()> {
        let request = abi::deferred::AdminSetRewardPoolCall {
            reward_pool: self.env.evm().reward_pool.0,
        }
        .encode();

        let client = EthRpcClient::new(self.env);
        client
            .send_raw_transaction(WalletName::Owner, self.env.evm().deferred, request.into())
            .await
    }
}
