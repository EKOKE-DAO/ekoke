use candid::Encode;
use did::ekoke::{Ekoke, EkokeResult};
use did::ID;
use icrc::icrc1::account::Account;

use crate::TestEnv;

pub struct EkokeRewardPoolClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for EkokeRewardPoolClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> EkokeRewardPoolClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn reserve_pool(
        &self,
        from: Account,
        contract_id: ID,
        amount: Ekoke,
    ) -> EkokeResult<Ekoke> {
        self.env
            .update(
                self.env.ekoke_reward_pool_id,
                from.owner,
                "reserve_pool",
                Encode!(&contract_id, &amount, &from.subaccount).unwrap(),
            )
            .unwrap()
    }

    pub fn send_reward(&self, contract_id: ID, amount: Ekoke, to: Account) -> EkokeResult<()> {
        self.env
            .update(
                self.env.ekoke_reward_pool_id,
                self.env.marketplace_id,
                "send_reward",
                Encode!(&contract_id, &amount, &to).unwrap(),
            )
            .unwrap()
    }
}
