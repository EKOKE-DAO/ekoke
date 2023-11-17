use async_trait::async_trait;
use candid::Principal;
use did::deferred::DeferredResult;
use did::fly::PicoFly;
use did::ID;

#[cfg(not(test))]
pub fn fly_client(principal: Principal) -> IcFlyClient {
    IcFlyClient { principal }
}

#[cfg(test)]
pub fn fly_client(_principal: Principal) -> IcFlyClient {
    IcFlyClient
}

#[async_trait]
pub trait FlyClient {
    /// Get contract reward. Returns $picoFly
    async fn get_contract_reward(
        &self,
        contract_id: ID,
        installments: u64,
    ) -> DeferredResult<PicoFly>;

    /// Notify fly canister that pool reward must be sent to the new owner with `picofly` value from contract id's pool
    async fn send_reward(
        &self,
        contract_id: ID,
        picofly: PicoFly,
        new_owner: Principal,
    ) -> DeferredResult<()>;
}

#[cfg(not(test))]
/// Fly canister client
pub struct IcFlyClient {
    principal: Principal,
}

#[cfg(test)]
#[derive(Default)]
pub struct IcFlyClient;

#[cfg(not(test))]
#[async_trait]
impl FlyClient for IcFlyClient {
    /// Get contract reward. Returns $picoFly
    async fn get_contract_reward(
        &self,
        _contract_id: ID,
        _installments: u64,
    ) -> DeferredResult<PicoFly> {
        todo!()
    }

    /// Send reward to new owner reducing the balance from the pool associated to the contract, for the value of picoFly
    async fn send_reward(
        &self,
        _contract_id: ID,
        _picofly: PicoFly,
        _new_owner: Principal,
    ) -> DeferredResult<()> {
        todo!()
    }
}

#[cfg(test)]
#[async_trait]
impl FlyClient for IcFlyClient {
    /// Get contract reward. Returns $picoFly
    async fn get_contract_reward(
        &self,
        _contract_id: ID,
        _installments: u64,
    ) -> DeferredResult<PicoFly> {
        Ok(71_000_u64.into())
    }

    /// Send reward to new owner reducing the balance from the pool associated to the contract, for the value of picoFly
    async fn send_reward(
        &self,
        _contract_id: ID,
        _picofly: PicoFly,
        _new_owner: Principal,
    ) -> DeferredResult<()> {
        Ok(())
    }
}
