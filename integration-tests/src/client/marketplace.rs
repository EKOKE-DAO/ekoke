use candid::{Encode, Principal};
use did::marketplace::MarketplaceResult;
use dip721::TokenIdentifier;
use icrc::icrc1::account::Subaccount;

use crate::TestEnv;

pub struct MarketplaceClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for MarketplaceClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> MarketplaceClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn get_token_price_icp(
        &self,
        caller: Principal,
        token_id: &TokenIdentifier,
    ) -> MarketplaceResult<u64> {
        let result: MarketplaceResult<u64> = self
            .env
            .update(
                self.env.marketplace_id,
                caller,
                "get_token_price_icp",
                Encode!(token_id).unwrap(),
            )
            .unwrap();

        result
    }

    pub fn buy_token(
        &self,
        caller: Principal,
        token_id: &TokenIdentifier,
        subaccount: &Option<Subaccount>,
    ) -> MarketplaceResult<()> {
        let result: MarketplaceResult<()> = self
            .env
            .update(
                self.env.marketplace_id,
                caller,
                "buy_token",
                Encode!(token_id, subaccount).unwrap(),
            )
            .unwrap();

        result
    }
}
