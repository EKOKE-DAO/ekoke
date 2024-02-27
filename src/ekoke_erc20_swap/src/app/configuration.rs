//! # Configuration
//!
//! Canister configuration

use std::cell::RefCell;

use candid::Principal;
use did::ekoke_erc20_swap::EthNetwork;
use did::{StorablePrincipal, H160};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};

use crate::app::memory::{
    CKETH_LEDGER_CANISTER_MEMORY_ID, CKETH_MINTER_CANISTER_MEMORY_ID,
    ERC20_BRIDGE_ADDRESS_MEMORY_ID, ETH_NETWORK_MEMORY_ID, LEDGER_CANISTER_MEMORY_ID,
    MEMORY_MANAGER,
};

thread_local! {
    /// Ekoke ledger canister
    static LEDGER_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(LEDGER_CANISTER_MEMORY_ID)),
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
    /// Set ledger canister address
    pub fn set_ledger_canister(canister_id: Principal) {
        LEDGER_CANISTER.with_borrow_mut(|cell| {
            cell.set(canister_id.into()).unwrap();
        });
    }

    /// Get ledger canister address
    #[allow(dead_code)]
    pub fn get_ledger_canister() -> Principal {
        LEDGER_CANISTER.with(|xrc| xrc.borrow().get().0)
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

    #[test]
    fn test_should_set_ledger_canister() {
        let principal =
            Principal::from_str("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap();
        Configuration::set_ledger_canister(principal);
        assert_eq!(Configuration::get_ledger_canister(), principal);
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
    }
}
