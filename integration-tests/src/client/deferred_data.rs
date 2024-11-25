use candid::{Encode, Principal};
use did::deferred::{Contract, DeferredDataResult, GenericValue};
use did::ID;

use crate::actor::admin;
use crate::TestEnv;

pub struct DeferredDataClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for DeferredDataClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> DeferredDataClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub async fn update_contract_property(
        &self,
        caller: Principal,
        id: ID,
        key: String,
        property: GenericValue,
    ) -> DeferredDataResult<()> {
        self.env
            .update(
                self.env.deferred_data,
                caller,
                "update_contract_property",
                Encode!(&id, &key, &property).unwrap(),
            )
            .await
            .unwrap()
    }

    pub async fn get_contracts(&self) -> Vec<ID> {
        let signed_contract: Vec<ID> = self
            .env
            .query(
                self.env.deferred_data,
                admin(),
                "get_contracts",
                Encode!(&()).unwrap(),
            )
            .await
            .unwrap();

        signed_contract
    }

    pub async fn get_contract(&self, contract_id: &ID) -> Option<Contract> {
        self.env
            .query(
                self.env.deferred_data,
                admin(),
                "get_contract",
                Encode!(contract_id).unwrap(),
            )
            .await
            .unwrap()
    }
}
