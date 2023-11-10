use async_trait::async_trait;
use candid::Principal;
use did::dilazionato::DilazionatoResult;
use did::ID;

#[cfg(not(test))]
pub fn fly_client(principal: Principal) -> IcFlyClient {
    IcFlyClient { principal }
}

#[cfg(test)]
pub fn fly_client(_principal: Principal) -> IcFlyClient {
    IcFlyClient::default()
}

#[async_trait]
pub trait FlyClient {
    /// Get contract reward. Returns $mFly
    async fn get_contract_reward(
        &self,
        contract_id: ID,
        installments: u64,
    ) -> DilazionatoResult<u64>;

    async fn send_reward(
        &self,
        contract_id: ID,
        mfly: u64,
        new_owner: Principal,
    ) -> DilazionatoResult<()>;
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
    /// Get contract reward. Returns $mFly
    async fn get_contract_reward(
        &self,
        _contract_id: ID,
        _installments: u64,
    ) -> DilazionatoResult<u64> {
        todo!()
    }

    /// Send reward to new owner reducing the balance from the pool associated to the contract, for the value of mFly
    async fn send_reward(
        &self,
        _contract_id: ID,
        _mfly: u64,
        _new_owner: Principal,
    ) -> DilazionatoResult<()> {
        todo!()
    }
}

#[cfg(test)]
#[async_trait]
impl FlyClient for IcFlyClient {
    /// Get contract reward. Returns $mFly
    async fn get_contract_reward(
        &self,
        _contract_id: ID,
        _installments: u64,
    ) -> DilazionatoResult<u64> {
        Ok(71_000)
    }

    /// Send reward to new owner reducing the balance from the pool associated to the contract, for the value of mFly
    async fn send_reward(
        &self,
        _contract_id: ID,
        _mfly: u64,
        _new_owner: Principal,
    ) -> DilazionatoResult<()> {
        Ok(())
    }
}
