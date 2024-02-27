mod eth_network;

use candid::{self, CandidType, Deserialize, Principal};
pub use eth_network::EthNetwork;

use crate::H160;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EkokeErc20SwapInitData {
    /// Canister administrators
    pub admins: Vec<Principal>,
    /// The canister ID of the CKETH ledger canister
    pub cketh_ledger_canister: Principal,
    /// The canister ID of the CKETH minter canister
    pub cketh_minter_canister: Principal,
    /// The Ethereum address of the ERC20 bridge
    pub erc20_bridge_address: H160,
    /// Initial ERC20 swap fee
    pub erc20_gas_price: u64,
    /// The Ethereum network
    pub erc20_network: EthNetwork,
    /// ID of ekoke-ledger canister
    pub ledger_id: Principal,
}
