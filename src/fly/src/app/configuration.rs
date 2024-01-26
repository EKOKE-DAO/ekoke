//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::fly::EthNetwork;
use did::{StorableAccount, StorablePrincipal, H160};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use icrc::icrc1::account::Account;

use crate::app::memory::{
    CKBTC_CANISTER_MEMORY_ID, CKETH_LEDGER_CANISTER_MEMORY_ID, CKETH_MINTER_CANISTER_MEMORY_ID,
    ERC20_BRIDGE_ADDRESS_MEMORY_ID, ETH_NETWORK_MEMORY_ID, ICP_LEDGER_CANISTER_MEMORY_ID,
    MEMORY_MANAGER, MINTING_ACCOUNT_MEMORY_ID, SWAP_ACCOUNT_MEMORY_ID, XRC_CANISTER_MEMORY_ID,
};

thread_local! {
    /// Minting account
    static MINTING_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(MINTING_ACCOUNT_MEMORY_ID)),
        Account {
            owner: Principal::anonymous(),
            subaccount: None
        }.into()).unwrap()
    );

    /// Swap account
    static SWAP_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(SWAP_ACCOUNT_MEMORY_ID)),
        Account {
            owner: Principal::anonymous(),
            subaccount: None
        }.into()).unwrap()
    );

    /// Xrc canister
    static XRC_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(XRC_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// Ckbtc canister
    static CKBTC_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(CKBTC_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// ICP ledger canister
    static ICP_LEDGER_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(ICP_LEDGER_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// ckEth ledger canister
    static CKETH_LEDGER_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(CKETH_LEDGER_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// ckEth minter canister
    static CKETH_MINTER_CANISTER : RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(CKETH_MINTER_CANISTER_MEMORY_ID)),
        Principal::anonymous().into()).unwrap()
    );

    /// ERC20 bridge address
    static ERC20_BRIDGE_ADDRESS: RefCell<StableCell<H160, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(ERC20_BRIDGE_ADDRESS_MEMORY_ID)),
        H160::zero()).unwrap()
    );

    /// Eth network
    static ETH_NETWORK: RefCell<StableCell<EthNetwork, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(ETH_NETWORK_MEMORY_ID)),
        EthNetwork::Ethereum).unwrap()
    );
}

/// canister configuration
pub struct Configuration;

impl Configuration {
    /// Set minting account
    pub fn set_minting_account(minting_account: Account) {
        MINTING_ACCOUNT.with_borrow_mut(|cell| {
            cell.set(minting_account.into()).unwrap();
        });
    }

    /// Set swap account
    pub fn set_swap_account(swap_account: Account) {
        SWAP_ACCOUNT.with_borrow_mut(|cell| {
            cell.set(swap_account.into()).unwrap();
        });
    }

    /// Get minting account address
    pub fn get_minting_account() -> Account {
        MINTING_ACCOUNT.with(|ma| ma.borrow().get().0)
    }

    /// Get swap account address
    pub fn get_swap_account() -> Account {
        SWAP_ACCOUNT.with(|sa| sa.borrow().get().0)
    }

    /// Set xrc canister address
    pub fn set_xrc_canister(canister_id: Principal) {
        XRC_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get xrc canister address
    #[allow(dead_code)]
    pub fn get_xrc_canister() -> Principal {
        XRC_CANISTER.with(|xrc| xrc.borrow().get().0)
    }

    /// Set ckbtc canister address
    pub fn set_ckbtc_canister(canister_id: Principal) {
        CKBTC_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get ckbtc canister address
    #[allow(dead_code)]
    pub fn get_ckbtc_canister() -> Principal {
        CKBTC_CANISTER.with(|ckbtc| ckbtc.borrow().get().0)
    }

    /// Set icp ledger canister address
    pub fn set_icp_ledger_canister(canister_id: Principal) {
        ICP_LEDGER_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get icp ledger canister address
    #[allow(dead_code)]
    pub fn get_icp_ledger_canister() -> Principal {
        ICP_LEDGER_CANISTER.with(|icp| icp.borrow().get().0)
    }

    /// Set cketh ledger canister address
    pub fn set_cketh_ledger_canister(canister_id: Principal) {
        CKETH_LEDGER_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get cketh ledger canister address
    #[allow(dead_code)]
    pub fn get_cketh_ledger_canister() -> Principal {
        CKETH_LEDGER_CANISTER.with(|cketh| cketh.borrow().get().0)
    }

    /// Set cketh minter canister address
    pub fn set_cketh_minter_canister(canister_id: Principal) {
        CKETH_MINTER_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get cketh minter canister address
    #[allow(dead_code)]
    pub fn get_cketh_minter_canister() -> Principal {
        CKETH_MINTER_CANISTER.with(|cketh| cketh.borrow().get().0)
    }

    /// Set erc20 bridge address
    pub fn set_erc20_bridge_address(address: H160) {
        ERC20_BRIDGE_ADDRESS.with_borrow_mut(|cell| {
            cell.set(address).unwrap();
        });
    }

    /// Get erc20 bridge address
    #[allow(dead_code)]
    pub fn get_erc20_bridge_address() -> H160 {
        ERC20_BRIDGE_ADDRESS.with(|erc20| erc20.borrow().get().clone())
    }

    /// Get eth network
    pub fn get_eth_network() -> EthNetwork {
        ETH_NETWORK.with(|eth_network| *eth_network.borrow().get())
    }

    /// Get eth chain id
    pub fn get_eth_chain_id() -> u64 {
        match Self::get_eth_network() {
            EthNetwork::Goerli => 5,
            EthNetwork::Ethereum => 1,
        }
    }

    /// Set eth network
    pub fn set_eth_network(network: EthNetwork) {
        ETH_NETWORK.with_borrow_mut(|cell| {
            cell.set(network).unwrap();
        });
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::bob_account;

    #[test]
    fn test_should_set_minting_account() {
        let minting_account = bob_account();
        Configuration::set_minting_account(minting_account);
        assert_eq!(Configuration::get_minting_account(), minting_account);
    }

    #[test]
    fn test_should_set_swap_account() {
        let swap_account = bob_account();
        Configuration::set_swap_account(swap_account);
        assert_eq!(Configuration::get_swap_account(), swap_account);
    }

    #[test]
    fn test_should_set_xrc_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_xrc_canister(principal);
        assert_eq!(Configuration::get_xrc_canister(), principal);
    }

    #[test]
    fn test_should_set_icp_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_icp_ledger_canister(principal);
        assert_eq!(Configuration::get_icp_ledger_canister(), principal);
    }

    #[test]
    fn test_should_set_ckbtc_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_ckbtc_canister(principal);
        assert_eq!(Configuration::get_ckbtc_canister(), principal);
    }

    #[test]
    fn test_should_set_cketh_ledger_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_cketh_ledger_canister(principal);
        assert_eq!(Configuration::get_cketh_ledger_canister(), principal);
    }

    #[test]
    fn test_should_set_cketh_minter_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_cketh_minter_canister(principal);
        assert_eq!(Configuration::get_cketh_minter_canister(), principal);
    }

    #[test]
    fn test_should_set_erc20_bridge_address() {
        let address = H160::from_hex_str("0x2CE04Fd64DB0372F6fb4B7a542f0F9196feE5663").unwrap();
        Configuration::set_erc20_bridge_address(address.clone());
        assert_eq!(Configuration::get_erc20_bridge_address(), address);
    }

    #[test]
    fn test_should_set_eth_network() {
        let network = EthNetwork::Goerli;
        Configuration::set_eth_network(network);
        assert_eq!(Configuration::get_eth_network(), network);
        assert_eq!(Configuration::get_eth_chain_id(), 5);
    }
}
