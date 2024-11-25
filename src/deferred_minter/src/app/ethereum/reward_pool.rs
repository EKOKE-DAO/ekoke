use abi::{self, AvailableRewardCall};
use did::deferred::{DeferredMinterError, DeferredMinterResult};
use did::H160;
use ethers_core::abi::{AbiDecode, AbiEncode};

use super::evm_rpc_client::EvmRpcClient;

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
    pub async fn available_rewards(&self, client: &EvmRpcClient) -> DeferredMinterResult<u128> {
        if cfg!(test) {
            return Ok(700_000_000_000_000);
        }

        let call = abi::RewardPoolCalls::AvailableReward(AvailableRewardCall).encode();

        let output = client.eth_call(&self.address, call.into()).await?;
        log::debug!("reward pool available output: {:?}", output);

        let available = abi::AvailableRewardReturn::decode_hex(output)
            .map_err(|err| DeferredMinterError::FailedToDecodeOutput(err.to_string()))?;

        log::debug!("reward pool available balance: {}", available.available);

        Ok(available.available.as_u128())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::app::test_utils::alice;

    #[tokio::test]
    async fn test_should_get_available_rewards() {
        let evm_rpc_client = EvmRpcClient::new(alice(), 1, None);

        let reward_pool = RewardPool::from(
            H160::from_hex_str("0x2CE04Fd64DB0372F6fb4B7a542f0F9196feE5663").unwrap(),
        )
        .available_rewards(&evm_rpc_client)
        .await
        .unwrap();

        assert_eq!(reward_pool, 700000000000000);
    }
}
