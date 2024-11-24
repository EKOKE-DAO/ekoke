use did::deferred::{DeferredMinterError, DeferredMinterResult};
use did::H160;
use ethers_core::abi::{AbiDecode, AbiEncode};

use super::evm_rpc_client::EvmRpcClient;
use crate::abi::{self, AvailableRewardCall};

pub struct RewardPool {
    address: H160,
}

impl From<H160> for RewardPool {
    fn from(address: H160) -> Self {
        Self { address }
    }
}

impl RewardPool {
    /// Get the available amount of reward tokens in the reward pool
    pub async fn available_rewards(&self, client: &EvmRpcClient) -> DeferredMinterResult<u64> {
        if cfg!(test) {
            return Ok(7_000_000_000_000);
        }

        let call = abi::RewardPoolCalls::AvailableReward(AvailableRewardCall).encode();

        let output = client.eth_call(&self.address, call.into()).await?;

        let available = abi::AvailableRewardReturn::decode(output)
            .map_err(|err| DeferredMinterError::FailedToDecodeOutput(err.to_string()))?;

        Ok(available.available.as_u64())
    }
}
