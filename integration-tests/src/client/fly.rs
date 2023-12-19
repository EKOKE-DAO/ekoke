use candid::Encode;
use did::fly::{FlyResult, PicoFly};
use did::ID;
use icrc::icrc1::account::Account;

use crate::TestEnv;

pub struct FlyClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for FlyClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> FlyClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn reserve_pool(
        &self,
        from: Account,
        contract_id: ID,
        picofly_amount: PicoFly,
    ) -> FlyResult<PicoFly> {
        let result: FlyResult<PicoFly> = self
            .env
            .update(
                self.env.fly_id,
                from.owner,
                "reserve_pool",
                Encode!(&from, &contract_id, &picofly_amount).unwrap(),
            )
            .unwrap();

        result
    }
}
