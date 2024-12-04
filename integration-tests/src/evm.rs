pub mod abi;
mod builder;
pub mod evm_rpc_did;
mod ganache;

use std::collections::HashMap;

use did::H160;
use ethers_signers::{LocalWallet, Signer};
use ganache::Ganache;
use testcontainers::ContainerAsync;

pub use self::builder::EvmBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WalletName {
    Owner,
    Alice,
    Bob,
    Charlie,
}

/// EVM test environment
pub struct Evm {
    /// Chain ID
    pub chain_id: u64,
    /// Parity container
    #[allow(unused)]
    container: ContainerAsync<Ganache>,
    /// Deferred contract
    pub deferred: H160,
    /// EKOKE token contract
    pub ekoke: H160,
    /// Reward pool contract
    pub reward_pool: H160,
    /// EVM HTTP endpoint url
    pub url: String,
    /// Wallets
    wallets: HashMap<WalletName, LocalWallet>,
}

impl Evm {
    /// get wallet by name
    pub fn get_wallet(&self, name: WalletName) -> &LocalWallet {
        self.wallets.get(&name).unwrap()
    }

    /// get eth address by name
    pub fn get_eth_address(&self, name: WalletName) -> H160 {
        self.get_wallet(name).address().into()
    }
}
