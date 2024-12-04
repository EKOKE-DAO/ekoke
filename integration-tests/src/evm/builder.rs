use std::collections::HashMap;
use std::sync::Arc;

use did::H160;
use ethers_middleware::SignerMiddleware;
use ethers_providers::Provider;
use ethers_signers::{LocalWallet, Signer};
use testcontainers::runners::AsyncRunner as _;

use super::{abi, Evm, WalletName};
use crate::evm::ganache::Ganache;

type EthHttpProvider = Provider<ethers_providers::Http>;

const OWNER_PRIVATE_KEY: &str =
    "0x5c8f13ca3206038a8d733b252d42ef2f1684708ccf843843caa5a258950698e8";
const ALICE_PRIVATE_KEY: &str =
    "0xd734768e633bdcc19b4b7c168685ef774f55fa76f539b763d48f9e7c1c5859af";
const BOB_PRIVATE_KEY: &str = "0x963f08abe882e45a42648b26cd83427b374b364232e4af9058a3c7c05babbe1b";
const CHARLIE_PRIVATE_KEY: &str =
    "0xf8dd8231fbe8fca1401c9c56571d35c11a4815fa16eff4adbf9b89cdd3fb4f93";

pub struct EvmBuilder;

impl EvmBuilder {
    /// Init evm test environment.
    ///
    /// Setups a local EVM node and installs the EVM-RPC canister.
    pub async fn setup() -> anyhow::Result<Evm> {
        // start container
        let container = Ganache::default().start().await?;
        let host_port = container.get_host_port_ipv4(8545).await?;
        let url = format!("http://localhost:{host_port}");
        println!("Running evm at {url}");

        let chain_id = Self::get_chain_id(&url).await;
        println!("Chain ID: {chain_id}",);

        // setup wallets
        let mut wallets = HashMap::new();
        wallets.insert(
            WalletName::Owner,
            Self::setup_wallet(OWNER_PRIVATE_KEY, chain_id).await,
        );
        wallets.insert(
            WalletName::Alice,
            Self::setup_wallet(ALICE_PRIVATE_KEY, chain_id).await,
        );
        wallets.insert(
            WalletName::Bob,
            Self::setup_wallet(BOB_PRIVATE_KEY, chain_id).await,
        );
        wallets.insert(
            WalletName::Charlie,
            Self::setup_wallet(CHARLIE_PRIVATE_KEY, chain_id).await,
        );

        // get owner
        let owner = wallets.get(&WalletName::Owner).unwrap();

        // install contracts
        let deferred = Self::install_deferred(owner, &url).await?;
        println!("Deferred contract: {deferred}");
        let ekoke = Self::install_ekoke(owner, &url).await?;
        println!("Ekoke contract: {ekoke}");
        let reward_pool = Self::install_reward_pool(owner, &url, &deferred, &ekoke).await?;
        println!("Reward pool contract: {reward_pool}");

        Ok(Evm {
            chain_id,
            container,
            deferred,
            ekoke,
            reward_pool,
            url,
            wallets,
        })
    }

    /// Setup a new wallet and add funds to it.
    async fn setup_wallet(private_key: &str, chain_id: u64) -> LocalWallet {
        private_key
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id)
    }

    async fn get_chain_id(rpc_url: &str) -> u64 {
        let response = reqwest::Client::new()
            .post(rpc_url)
            .json(&serde_json::json!(
                {
                    "method": "eth_chainId",
                    "params": [],
                    "id": 1,
                    "jsonrpc": "2.0"
                }
            ))
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success(), "Failed to get chain id");

        let body = response.json::<serde_json::Value>().await.unwrap();
        let chain_id_str = body["result"].as_str().unwrap();

        u64::from_str_radix(chain_id_str.trim_start_matches("0x"), 16).unwrap()
    }

    /// Install the deferred contract
    async fn install_deferred(wallet: &LocalWallet, rpc_url: &str) -> anyhow::Result<H160> {
        let provider = EthHttpProvider::try_from(rpc_url)?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

        Ok(abi::Deferred::deploy(client, wallet.address())?
            .send()
            .await?
            .address()
            .into())
    }

    /// Install the ekoke contract
    async fn install_ekoke(wallet: &LocalWallet, rpc_url: &str) -> anyhow::Result<H160> {
        let provider = EthHttpProvider::try_from(rpc_url)?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

        Ok(abi::Ekoke::deploy(client, wallet.address())?
            .send()
            .await?
            .address()
            .into())
    }

    /// Install the reward pool contract
    async fn install_reward_pool(
        wallet: &LocalWallet,
        rpc_url: &str,
        deferred: &H160,
        ekoke: &H160,
    ) -> anyhow::Result<H160> {
        let provider = EthHttpProvider::try_from(rpc_url)?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

        Ok(
            abi::RewardPool::deploy(client, (wallet.address(), ekoke.0, deferred.0))?
                .send()
                .await?
                .address()
                .into(),
        )
    }
}
