use abi::{self, CloseContractCall, CreateContractCall, CreateContractRequest, SellerRequest};
use did::deferred::{Contract, DeferredMinterResult};
use did::{H160, ID};
use ethers_core::abi::AbiEncode;
use ethers_core::types::{Bytes, TransactionRequest};
use num_traits::cast::ToPrimitive;

use super::evm_rpc_client::EvmRpcClient;
use super::Wallet;
use crate::app::configuration::Configuration;

/// Gas required for `createContract`
const CREATE_CONTRACT_GAS: u64 = 500_000;
/// Gas required for `closeContract`
const CLOSE_CONTRACT_GAS: u64 = 40_000;

pub struct DeferredErc721 {
    address: H160,
}

impl From<H160> for DeferredErc721 {
    fn from(address: H160) -> Self {
        Self { address }
    }
}

impl DeferredErc721 {
    /// Create a new contract on the Deferred Erc721 contract
    pub async fn create_contract(
        &self,
        wallet: &Wallet,
        evm_rpc_client: &EvmRpcClient,
        contract: &Contract,
        reward: Option<u128>,
        token_price_usd: u64,
    ) -> DeferredMinterResult<()> {
        if cfg!(test) {
            return Ok(());
        }

        let deferred_data_principal = Configuration::get_deferred_data_canister().to_text();
        let metadata_uri = format!(
            "https://{deferred_data_principal}.raw.icp0.io/contract/{}",
            contract.id
        );

        let contract_id = contract.id.0.to_u64().expect("Contract ID is too large");

        let payload = abi::DeferredCalls::CreateContract(CreateContractCall {
            request: CreateContractRequest {
                contract_id: contract_id.into(),
                metadata_uri,
                sellers: contract
                    .sellers
                    .iter()
                    .map(|seller| SellerRequest {
                        seller: seller.address.0,
                        quota: seller.quota,
                    })
                    .collect(),
                buyers: contract.buyers.iter().map(|buyer| buyer.0).collect(),
                ekoke_reward: reward.unwrap_or_default().into(),
                token_price_usd: token_price_usd.into(),
                tokens_amount: contract.installments.into(),
            },
        })
        .encode();

        self.send_tx(wallet, evm_rpc_client, payload, CREATE_CONTRACT_GAS)
            .await
    }

    /// Close a contract on the Deferred Erc721 contract
    pub async fn close_contract(
        &self,
        wallet: &Wallet,
        evm_rpc_client: &EvmRpcClient,
        contract_id: ID,
    ) -> DeferredMinterResult<()> {
        if cfg!(test) {
            return Ok(());
        }

        let contract_id = contract_id.0.to_u64().expect("Contract ID is too large");

        let payload = abi::DeferredCalls::CloseContract(CloseContractCall {
            contract_id: contract_id.into(),
        })
        .encode();

        self.send_tx(wallet, evm_rpc_client, payload, CLOSE_CONTRACT_GAS)
            .await
    }

    async fn send_tx(
        &self,
        wallet: &Wallet,
        evm_rpc_client: &EvmRpcClient,
        payload: Vec<u8>,
        gas: u64,
    ) -> DeferredMinterResult<()> {
        let eth_address = wallet.address().await?;
        let nonce = evm_rpc_client.get_next_nonce(eth_address.clone()).await?;

        let tx = TransactionRequest {
            from: Some(eth_address.0),
            to: Some(self.address.0.into()),
            value: None,
            gas: Some(gas.into()),
            gas_price: Some(Configuration::get_gas_price().into()),
            data: Some(Bytes::from(payload.encode())),
            nonce: Some(nonce),
            chain_id: Some(Configuration::get_chain_id().into()),
        };

        // sign and send the transaction
        let signed_tx = wallet.sign_transaction(tx).await?;

        evm_rpc_client.eth_send_raw_transaction(signed_tx).await?;

        Ok(())
    }
}
