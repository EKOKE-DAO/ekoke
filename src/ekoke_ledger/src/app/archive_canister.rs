use did::ekoke::EkokeResult;
use did::ekoke_index::Transaction;

pub struct ArchiveCanister;

impl ArchiveCanister {
    #[cfg(target_family = "wasm")]
    pub async fn commit(tx: Transaction) -> EkokeResult<u64> {
        let principal = super::configuration::Configuration::get_archive_canister();

        let (tx_id,): (u64,) = ic_cdk::call(principal, "commit", (tx,))
            .await
            .map_err(|(code, msg)| did::ekoke::EkokeError::CanisterCall(code, msg))?;

        Ok(tx_id)
    }

    #[cfg(not(target_family = "wasm"))]
    #[allow(unused_variables)]
    pub async fn commit(tx: Transaction) -> EkokeResult<u64> {
        Ok(0u64)
    }
}
