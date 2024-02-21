use candid::Encode;
use did::ekoke_index::Transaction;

use crate::actor::admin;
use crate::TestEnv;

pub struct EkokeArchiveClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> From<&'a TestEnv> for EkokeArchiveClient<'a> {
    fn from(env: &'a TestEnv) -> Self {
        Self::new(env)
    }
}

impl<'a> EkokeArchiveClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn get_transaction(&self, id: u64) -> Option<Transaction> {
        self.env
            .update(
                self.env.ekoke_archive_id,
                admin(),
                "get_transaction",
                Encode!(&id).unwrap(),
            )
            .unwrap()
    }
}
