use candid::{Encode, Principal};
use did::deferred::{Agency, ContractRegistration, DeferredMinterResult, RealEstate};
use did::{H160, ID};

use crate::actor::admin;
use crate::TestEnv;

pub struct DeferredMinterClient<'a, T>
where
    T: TestEnv,
{
    pub env: &'a T,
}

impl<'a, T> DeferredMinterClient<'a, T>
where
    T: TestEnv,
{
    pub fn new(env: &'a T) -> Self {
        Self { env }
    }

    pub async fn get_eth_address(&self) -> DeferredMinterResult<H160> {
        let res: DeferredMinterResult<String> = self
            .env
            .update(
                self.env.deferred_minter(),
                admin(),
                "get_eth_address",
                Encode!(&()).unwrap(),
            )
            .await
            .expect("Failed to get eth address");

        match res.map(|address| H160::from_hex_str(&address)) {
            Ok(Ok(address)) => Ok(address),
            Ok(Err(err)) => panic!("Failed to parse address: {}", err),
            Err(err) => Err(err),
        }
    }

    pub async fn create_contract(
        &self,
        caller: Principal,
        data: ContractRegistration,
    ) -> DeferredMinterResult<ID> {
        let contract_id: DeferredMinterResult<ID> = self
            .env
            .update(
                self.env.deferred_minter(),
                caller,
                "create_contract",
                Encode!(&data).unwrap(),
            )
            .await
            .expect("Failed to create contract");

        contract_id
    }

    pub async fn close_contract(
        &self,
        caller: Principal,
        contract_id: ID,
    ) -> DeferredMinterResult<()> {
        let res: DeferredMinterResult<()> = self
            .env
            .update(
                self.env.deferred_minter(),
                caller,
                "close_contract",
                Encode!(&contract_id).unwrap(),
            )
            .await
            .expect("Failed to close contract");

        res
    }

    pub async fn create_real_estate(
        &self,
        caller: Principal,
        real_estate: RealEstate,
    ) -> DeferredMinterResult<ID> {
        let res: DeferredMinterResult<ID> = self
            .env
            .update(
                self.env.deferred_minter(),
                caller,
                "create_real_estate",
                Encode!(&real_estate).unwrap(),
            )
            .await
            .expect("Failed to create real estate");

        res
    }

    pub async fn delete_real_estate(
        &self,
        caller: Principal,
        real_estate_id: ID,
    ) -> DeferredMinterResult<()> {
        let res: DeferredMinterResult<()> = self
            .env
            .update(
                self.env.deferred_minter(),
                caller,
                "delete_real_estate",
                Encode!(&real_estate_id).unwrap(),
            )
            .await
            .expect("Failed to delete real estate");

        res
    }

    pub async fn update_real_estate(
        &self,
        caller: Principal,
        id: ID,
        data: RealEstate,
    ) -> DeferredMinterResult<()> {
        let res: DeferredMinterResult<()> = self
            .env
            .update(
                self.env.deferred_minter(),
                caller,
                "update_real_estate",
                Encode!(&id, &data).unwrap(),
            )
            .await
            .expect("Failed to update real estate");

        res
    }

    pub async fn set_custodians(&self, principals: Vec<Principal>) {
        self.env
            .update::<()>(
                self.env.deferred_minter(),
                admin(),
                "admin_set_custodians",
                Encode!(&principals).unwrap(),
            )
            .await
            .expect("Failed to set custodians");
    }

    pub async fn admin_register_agency(&self, wallet: Principal, agency: Agency) {
        let _: () = self
            .env
            .update(
                self.env.deferred_minter(),
                admin(),
                "admin_register_agency",
                Encode!(&wallet, &agency).unwrap(),
            )
            .await
            .expect("Failed to register agency");
    }

    pub async fn remove_agency(&self, wallet: Principal) -> DeferredMinterResult<()> {
        self.env
            .update(
                self.env.deferred_minter(),
                wallet,
                "remove_agency",
                Encode!(&wallet).unwrap(),
            )
            .await
            .expect("Failed to remove agency")
    }
}
