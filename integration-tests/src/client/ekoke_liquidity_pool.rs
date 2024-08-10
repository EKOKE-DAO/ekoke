use candid::{Encode, Nat, Principal};
use did::ekoke_liquidity_pool::WithdrawError;
use icrc::icrc1::account::{Account, Subaccount};

use crate::actor::admin;
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

    pub fn admin_withdraw_icp(&self, to: Account, amount: Nat) -> Result<(), WithdrawError> {
        self.env
            .update(
                self.env.ekoke_liquidity_pool_id,
                admin(),
                "admin_withdraw_icp",
                Encode!(&to, &amount).unwrap(),
            )
            .unwrap()
    }
}
