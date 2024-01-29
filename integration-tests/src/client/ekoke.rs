use candid::Encode;
use did::ekoke::{EkokeResult, PicoEkoke};
use did::ID;
use icrc::icrc1::account::Account;

use crate::TestEnv;

pub struct EkokeClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for EkokeClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> EkokeClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn reserve_pool(
        &self,
        from: Account,
        contract_id: ID,
        picoekoke_amount: PicoEkoke,
    ) -> EkokeResult<PicoEkoke> {
        let result: EkokeResult<PicoEkoke> = self
            .env
            .update(
                self.env.ekoke_id,
                from.owner,
                "reserve_pool",
                Encode!(&from, &contract_id, &picoekoke_amount).unwrap(),
            )
            .unwrap();

        result
    }
}
