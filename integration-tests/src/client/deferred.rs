use candid::{Encode, Nat, Principal};
use did::deferred::{ContractRegistration, DeferredResult};
use did::ID;
use dip721::{NftError, TokenMetadata};

use crate::actor::{admin, alice};
use crate::TestEnv;

pub struct DeferredClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for DeferredClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> DeferredClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn register_contract(&self, data: ContractRegistration) -> DeferredResult<ID> {
        let contract_id: DeferredResult<ID> = self
            .env
            .update(
                self.env.deferred_id,
                admin(),
                "register_contract",
                Encode!(&data).unwrap(),
            )
            .unwrap();

        contract_id
    }

    pub fn admin_sign_contract(&self, id: Nat) -> DeferredResult<()> {
        let res: DeferredResult<()> = self
            .env
            .update(
                self.env.deferred_id,
                admin(),
                "admin_sign_contract",
                Encode!(&id).unwrap(),
            )
            .unwrap();

        res
    }

    pub fn update_contract_buyers(
        &self,
        caller: Principal,
        contract_id: ID,
        buyers: Vec<Principal>,
    ) -> DeferredResult<()> {
        let res: DeferredResult<()> = self
            .env
            .update(
                self.env.deferred_id,
                caller,
                "update_contract_buyers",
                Encode!(&contract_id, &buyers).unwrap(),
            )
            .unwrap();

        res
    }

    pub fn get_signed_contracts(&self) -> Vec<ID> {
        let signed_contract: Vec<ID> = self
            .env
            .query(
                self.env.deferred_id,
                admin(),
                "get_signed_contracts",
                Encode!(&()).unwrap(),
            )
            .unwrap();

        signed_contract
    }

    pub fn total_supply(&self) -> Nat {
        let total_supply: Nat = self
            .env
            .query(
                self.env.deferred_id,
                admin(),
                "total_supply",
                Encode!(&()).unwrap(),
            )
            .unwrap();

        total_supply
    }

    pub fn token_metadata(&self, token_id: Nat) -> Result<TokenMetadata, NftError> {
        let token: Result<TokenMetadata, NftError> = self
            .env
            .query(
                self.env.deferred_id,
                alice(),
                "token_metadata",
                Encode!(&token_id).unwrap(),
            )
            .unwrap();

        token
    }

    pub fn admin_get_unsigned_contracts(&self) -> Vec<ID> {
        let unsigned_contracts: Vec<ID> = self
            .env
            .query(
                self.env.deferred_id,
                admin(),
                "admin_get_unsigned_contracts",
                Encode!(&()).unwrap(),
            )
            .unwrap();

        unsigned_contracts
    }

    pub fn set_custodians(&self, principals: Vec<Principal>) {
        self.env
            .update::<()>(
                self.env.deferred_id,
                admin(),
                "set_custodians",
                Encode!(&principals).unwrap(),
            )
            .unwrap();
    }
}
