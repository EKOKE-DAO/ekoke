use candid::Principal;
use did::H160;

pub struct Evm {
    pub deferred: H160,
    pub reward_pool: H160,
    pub url: String,
}

impl Evm {
    pub async fn setup(evm_rpc_canister: Principal) -> Self {
        todo!()
    }
}
