use did::ekoke::EkokeResult;
use did::ekoke_index::{Transaction, TxId};

pub struct IndexCanister;

impl IndexCanister {
    #[cfg(target_family = "wasm")]
    pub async fn commit(tx: Transaction) -> EkokeResult<TxId> {
        let principal = super::configuration::Configuration::get_index_canister();

        let (tx_id,): (TxId,) = ic_cdk::call(principal, "commit", (tx,))
            .await
            .map_err(|(code, msg)| did::ekoke::EkokeError::CanisterCall(code, msg))?;

        Ok(tx_id)
    }

    #[cfg(not(target_family = "wasm"))]
    #[allow(unused_variables)]
    pub async fn commit(tx: Transaction) -> EkokeResult<TxId> {
        Ok(0u64.into())
    }
}
