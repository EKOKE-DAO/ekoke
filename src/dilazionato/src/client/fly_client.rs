use candid::Principal;
use did::dilazionato::SellContractResult;
use did::ID;

/// Fly canister client
pub struct FlyClient {
    principal: Principal,
}

impl From<Principal> for FlyClient {
    fn from(value: Principal) -> Self {
        Self { principal: value }
    }
}

impl FlyClient {
    /// Get contract reward. Returns $mFly
    pub async fn get_contract_reward(
        &self,
        _contract_id: ID,
        _installments: u64,
    ) -> SellContractResult<u64> {
        todo!()
    }

    /// Send reward to new owner reducing the balance from the pool associated to the contract, for the value of mFly
    pub async fn send_reward(
        &self,
        _contract_id: ID,
        _mfly: u64,
        _new_owner: Principal,
    ) -> SellContractResult<()> {
        todo!()
    }
}
