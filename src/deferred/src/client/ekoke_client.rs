use async_trait::async_trait;
use candid::Principal;
use did::deferred::DeferredResult;
use did::ekoke::PicoEkoke;
use did::ID;

#[cfg(not(test))]
pub fn ekoke_client(principal: Principal) -> IcEkokeClient {
    IcEkokeClient { principal }
}

#[cfg(test)]
pub fn ekoke_client(_principal: Principal) -> IcEkokeClient {
    IcEkokeClient
}

#[async_trait]
pub trait EkokeClient {
    /// Get contract reward. Returns $picoEkoke
    async fn get_contract_reward(
        &self,
        contract_id: ID,
        installments: u64,
    ) -> DeferredResult<PicoEkoke>;
}

#[cfg(not(test))]
/// Ekoke canister client
pub struct IcEkokeClient {
    principal: Principal,
}

#[cfg(test)]
#[derive(Default)]
pub struct IcEkokeClient;

#[cfg(not(test))]
#[async_trait]
impl EkokeClient for IcEkokeClient {
    /// Get contract reward. Returns $picoEkoke
    async fn get_contract_reward(
        &self,
        contract_id: ID,
        installments: u64,
    ) -> did::deferred::DeferredResult<PicoEkoke> {
        let result: (did::ekoke::EkokeResult<PicoEkoke>,) = ic_cdk::call(
            self.principal,
            "get_contract_reward",
            (contract_id, installments),
        )
        .await
        .map_err(|(code, err)| did::deferred::DeferredError::CanisterCall(code, err))?;

        let reward = result.0?;
        Ok(reward)
    }
}

#[cfg(test)]
#[async_trait]
impl EkokeClient for IcEkokeClient {
    /// Get contract reward. Returns $picoEkoke
    async fn get_contract_reward(
        &self,
        _contract_id: ID,
        _installments: u64,
    ) -> DeferredResult<PicoEkoke> {
        Ok(71_000__u64.into())
    }
}
