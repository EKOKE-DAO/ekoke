use did::ekoke_index::{Transaction, TxId};

pub struct IndexCanister;

impl IndexCanister {
    #[cfg(target_family = "wasm")]
    pub async fn commit(id: u64, tx: Transaction) -> TxId {
        let principal = super::configuration::Configuration::get_index_canister();

        let (tx_id,): (TxId,) = ic_cdk::call(principal, "commit", (id, tx)).await.unwrap();

        tx_id
    }

    #[cfg(not(target_family = "wasm"))]
    #[allow(unused_variables)]
    pub async fn commit(id: u64, tx: Transaction) -> TxId {
        id.into()
    }
}
