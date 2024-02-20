use candid::Encode;
use did::ekoke_index::{GetAccountTransactionArgs, GetTransactionsResult};

use crate::actor::admin;
use crate::TestEnv;

pub struct EkokeIndexClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for EkokeIndexClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> EkokeIndexClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn get_account_transactions(
        &self,
        args: GetAccountTransactionArgs,
    ) -> GetTransactionsResult {
        self.env
            .update(
                self.env.ekoke_index_id,
                admin(),
                "get_account_transactions",
                Encode!(&args).unwrap(),
            )
            .unwrap()
    }
}
