use did::ekoke_index::Transaction;

pub struct ArchiveCanister;

impl ArchiveCanister {
    #[cfg(target_family = "wasm")]
    pub async fn get_transaction(id: u64) -> Result<Option<Transaction>, String> {
        let principal = super::configuration::Configuration::get_archive_canister();

        let (txs,): (Option<Transaction>,) = ic_cdk::call(principal, "get_transaction", (id,))
            .await
            .map_err(|(_, msg)| msg)?;

        Ok(txs)
    }

    #[cfg(not(target_family = "wasm"))]
    #[allow(unused_variables)]
    pub async fn get_transaction(id: u64) -> Result<Option<Transaction>, String> {
        Ok(Some(Transaction {
            kind: "transfer".to_string(),
            mint: None,
            burn: None,
            transfer: None,
            approve: None,
            timestamp: 0,
        }))
    }
}
