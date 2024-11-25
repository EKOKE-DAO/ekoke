mod deferred;

use std::time::Instant;

use did::H160;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Bytes, TransactionRequest};
use ethers_signers::Signer;

pub use self::deferred::DeferredErc721Client;
use crate::evm::WalletName;
use crate::TestEnv;

pub struct EthRpcClient<'a, T: TestEnv> {
    env: &'a T,
}

impl<'a, T> EthRpcClient<'a, T>
where
    T: TestEnv,
{
    pub fn new(env: &'a T) -> Self {
        Self { env }
    }

    /// Sends ETH from one wallet to another
    pub async fn send_eth(&self, from: WalletName, to: H160, amount: u64) -> anyhow::Result<()> {
        let from = self.env.evm().get_wallet(from);

        // make transaction
        let transaction = TransactionRequest {
            from: from.address().into(),
            to: Some(to.0.into()),
            value: Some(amount.into()),
            gas: Some(21000.into()),
            gas_price: Some(20_000_000_000u64.into()),
            ..Default::default()
        };
        let transaction: TypedTransaction = transaction.into();

        // sign transaction
        let signature = from.sign_transaction(&transaction).await?;
        let signed_tx = transaction.rlp_signed(&signature);

        // send transaction
        let response = reqwest::Client::new()
            .post(&self.env.evm().url)
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": "eth_sendRawTransaction",
                "params": [signed_tx],
                "id": 1
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to send transaction: {:?}", response.text().await?);
        }

        // get transaction hash
        let response = response.json::<serde_json::Value>().await?;

        println!("Send ETH tx; response: {response:?}",);
        let tx_hash = response["result"].as_str().unwrap();

        println!("Transfer Transaction hash: {}", tx_hash);

        // get tx receipt and check for block
        let start = Instant::now();
        loop {
            let response = reqwest::Client::new()
                .post(&self.env.evm().url)
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getTransactionReceipt",
                    "params": [tx_hash],
                    "id": 1
                }))
                .send()
                .await?;

            if !response.status().is_success() {
                if start.elapsed().as_secs() > 30 {
                    anyhow::bail!("Transaction not mined after 30 seconds");
                }

                anyhow::bail!(
                    "Failed to get transaction receipt: {:?}",
                    response.text().await?
                );
            }

            let response = response.json::<serde_json::Value>().await?;
            if response["result"].is_null() {
                println!("Waiting for transaction to be mined...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }

            let block_number = response["result"]["blockNumber"].as_str().unwrap();
            println!("Transaction mined in block: {}", block_number);
            break;
        }

        Ok(())
    }

    async fn get_nonce(&self, wallet: WalletName) -> anyhow::Result<u64> {
        let wallet = self.env.evm().get_wallet(wallet);

        let wallet_address = H160::from(wallet.address());

        let response = reqwest::Client::new()
            .post(&self.env.evm().url)
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionCount",
                "params": [wallet_address.to_hex_str(), "pending"],
                "id": 1
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get nonce: {:?}", response.text().await?);
        }

        let response = response.json::<serde_json::Value>().await?;
        let nonce = u64::from_str_radix(
            response["result"]
                .as_str()
                .unwrap()
                .trim_start_matches("0x"),
            16,
        )?;

        Ok(nonce)
    }

    /// Sends ETH from one wallet to another
    pub async fn send_raw_transaction(
        &self,
        from: WalletName,
        to: H160,
        data: Bytes,
    ) -> anyhow::Result<()> {
        let nonce = self.get_nonce(from).await?;
        let from = self.env.evm().get_wallet(from);

        // make transaction
        let transaction = TransactionRequest {
            from: from.address().into(),
            to: Some(to.0.into()),
            value: None,
            data: Some(data),
            gas: Some(100_000.into()),
            gas_price: Some(20_000_000_000u64.into()),
            nonce: Some(nonce.into()),
            ..Default::default()
        };
        let transaction: TypedTransaction = transaction.into();

        // sign transaction
        let signature = from.sign_transaction(&transaction).await?;
        let signed_tx = transaction.rlp_signed(&signature);

        // send transaction
        let response = reqwest::Client::new()
            .post(&self.env.evm().url)
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": "eth_sendRawTransaction",
                "params": [signed_tx],
                "id": 1
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to send transaction: {:?}", response.text().await?);
        }

        // get transaction hash
        let response = response.json::<serde_json::Value>().await?;

        println!("Send ETH tx; response: {response:?}",);
        let tx_hash = response["result"].as_str().unwrap();

        println!("Raw Transaction hash: {}", tx_hash);

        // get tx receipt and check for block
        let start = Instant::now();
        loop {
            let response = reqwest::Client::new()
                .post(&self.env.evm().url)
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getTransactionReceipt",
                    "params": [tx_hash],
                    "id": 1
                }))
                .send()
                .await?;

            if !response.status().is_success() {
                if start.elapsed().as_secs() > 30 {
                    anyhow::bail!("Transaction not mined after 30 seconds");
                }

                anyhow::bail!(
                    "Failed to get transaction receipt: {:?}",
                    response.text().await?
                );
            }

            let response = response.json::<serde_json::Value>().await?;
            if response["result"].is_null() {
                println!("Waiting for transaction to be mined...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }

            let block_number = response["result"]["blockNumber"].as_str().unwrap();
            println!("Transaction mined in block: {}", block_number);
            break;
        }

        Ok(())
    }

    async fn eth_call(&self, to: H160, data: String) -> anyhow::Result<String> {
        let response = reqwest::Client::new()
            .post(&self.env.evm().url)
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": "eth_call",
                "params": [{
                    "to": to,
                    "data": data,
                }, "latest"],
                "id": 1
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to call contract: {:?}", response.text().await?);
        }

        let response = response.json::<serde_json::Value>().await?;
        let result = response["result"].as_str().unwrap();

        Ok(result.to_string())
    }
}
