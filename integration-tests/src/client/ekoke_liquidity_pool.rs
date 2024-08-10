use candid::{Encode, Principal};
use did::ekoke_liquidity_pool::WithdrawError;
use icrc::icrc1::account::Subaccount;

use crate::TestEnv;

pub struct EkokeLiquidityPoolClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for EkokeLiquidityPoolClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> EkokeLiquidityPoolClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn withdraw_refund(
        &self,
        caller: Principal,
        subaccount: Option<Subaccount>,
    ) -> Result<(), WithdrawError> {
        self.env
            .update(
                self.env.ekoke_liquidity_pool_id,
                caller,
                "withdraw_refund",
                Encode!(&subaccount).unwrap(),
            )
            .unwrap()
    }
}
