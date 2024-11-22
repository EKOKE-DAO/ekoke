use candid::{Encode, Principal};
use did::deferred::{Agency, ContractRegistration, DeferredMinterResult};
use did::ID;

use crate::actor::admin;
use crate::TestEnv;

pub struct DeferredMinterClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for DeferredMinterClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> DeferredMinterClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn create_contract(
        &self,
        caller: Principal,
        data: ContractRegistration,
    ) -> DeferredMinterResult<ID> {
        let contract_id: DeferredMinterResult<ID> = self
            .env
            .update(
                self.env.deferred_minter,
                caller,
                "create_contract",
                Encode!(&data).unwrap(),
            )
            .unwrap();

        contract_id
    }

    pub fn close_contract(&self, caller: Principal, contract_id: ID) -> DeferredMinterResult<()> {
        let res: DeferredMinterResult<()> = self
            .env
            .update(
                self.env.deferred_minter,
                caller,
                "close_contract",
                Encode!(&contract_id).unwrap(),
            )
            .unwrap();

        res
    }

    pub fn set_custodians(&self, principals: Vec<Principal>) {
        self.env
            .update::<()>(
                self.env.deferred_minter,
                admin(),
                "admin_set_custodians",
                Encode!(&principals).unwrap(),
            )
            .unwrap();
    }

    pub fn admin_register_agency(&self, wallet: Principal, agency: Agency) {
        let _: () = self
            .env
            .update(
                self.env.deferred_minter,
                admin(),
                "admin_register_agency",
                Encode!(&wallet, &agency).unwrap(),
            )
            .unwrap();
    }

    pub fn remove_agency(&self, wallet: Principal) -> DeferredMinterResult<()> {
        self.env
            .update(
                self.env.deferred_minter,
                wallet,
                "remove_agency",
                Encode!(&wallet).unwrap(),
            )
            .unwrap()
    }
}
