mod currency;

use std::cell::RefCell;
use std::str::FromStr as _;

use candid::Principal;
use did::deferred::{DeferredMinterError, DeferredMinterResult, EcdsaKey};
use did::{StorablePrincipal, H160};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, StableVec};

use self::currency::Currency;
use crate::app::memory::{
    ALLOWED_CURRENCIES_MEMORY_ID, CHAIN_ID_MEMORY_ID, DEFERRED_DATA_CANISTER_MEMORY_ID,
    DEFERRED_ERC721_CONTRACT_MEMORY_ID, ECDSA_KEY_MEMORY_ID, EVM_CUSTOM_RPC_API_MEMORY_ID,
    EVM_GAS_PRICE_MEMORY_ID, EVM_RPC_MEMORY_ID, MEMORY_MANAGER, REWARD_POOL_CONTRACT_MEMORY_ID,
};

const DEFAULT_GAS_PRICE: u64 = 20_000_000_000;

thread_local! {
    /// Ekoke Canister principal
    static DEFERRED_DATA_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(DEFERRED_DATA_CANISTER_MEMORY_ID)), Principal::anonymous().into()).unwrap()
    );

    /// ETH address of deferred NFT contract
    static DEFERRED_ERC721_CONTRACT: RefCell<StableCell<H160, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(DEFERRED_ERC721_CONTRACT_MEMORY_ID)), H160::zero()).unwrap()
    );

    /// ETH address of ekoke ERC20 contract
    static REWARD_POOL_CONTRACT: RefCell<StableCell<H160, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(REWARD_POOL_CONTRACT_MEMORY_ID)), H160::zero()).unwrap()
    );

    /// Allowed currencies
    static ALLOWED_CURRENCIES: RefCell<StableVec<Currency, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableVec::new(MEMORY_MANAGER.with(|mm| mm.get(ALLOWED_CURRENCIES_MEMORY_ID))).unwrap()
    );

    /// Ecdsa key
    static ECDSA_KEY: RefCell<StableCell<u8, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(ECDSA_KEY_MEMORY_ID)), 0).unwrap()
    );

    /// chain id
    static CHAIN_ID: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(CHAIN_ID_MEMORY_ID)), 0).unwrap()
    );

    /// Ekoke Canister principal
    static EVM_RPC: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(EVM_RPC_MEMORY_ID)), Principal::anonymous().into()).unwrap()
    );

    static EVM_RPC_API: RefCell<StableCell<Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(EVM_CUSTOM_RPC_API_MEMORY_ID)), vec![]).unwrap()
    );

    /// gas price
    static GAS_PRICE: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(EVM_GAS_PRICE_MEMORY_ID)), DEFAULT_GAS_PRICE).unwrap()
    );


}

pub struct Configuration;

impl Configuration {
    pub fn get_deferred_data_canister() -> Principal {
        DEFERRED_DATA_CANISTER.with_borrow(|cell| cell.get().0)
    }

    pub fn set_deferred_data_canister(principal: Principal) -> DeferredMinterResult<()> {
        DEFERRED_DATA_CANISTER.with_borrow_mut(|cell| {
            cell.set(principal.into())
                .map_err(|_| DeferredMinterError::StorageError)
        })?;

        Ok(())
    }

    pub fn get_deferred_erc721_contract() -> H160 {
        DEFERRED_ERC721_CONTRACT.with_borrow(|cell| *cell.get())
    }

    pub fn set_deferred_erc721_contract(address: H160) -> DeferredMinterResult<()> {
        DEFERRED_ERC721_CONTRACT.with_borrow_mut(|cell| {
            cell.set(address)
                .map_err(|_| DeferredMinterError::StorageError)
        })?;

        Ok(())
    }

    pub fn get_reward_pool_contract() -> H160 {
        REWARD_POOL_CONTRACT.with_borrow(|cell| *cell.get())
    }

    pub fn set_reward_pool_contract(address: H160) -> DeferredMinterResult<()> {
        REWARD_POOL_CONTRACT.with_borrow_mut(|cell| {
            cell.set(address)
                .map_err(|_| DeferredMinterError::StorageError)
        })?;

        Ok(())
    }

    /// Set allowed currencies
    pub fn set_allowed_currencies(currencies: Vec<String>) {
        let currencies = currencies
            .iter()
            .map(|currency| Currency::from_str(currency).expect("Invalid currency"))
            .collect::<Vec<Currency>>();

        ALLOWED_CURRENCIES.with_borrow_mut(|cell| {
            let items = cell.len();
            for _ in 0..items {
                cell.pop();
            }

            for currency in currencies {
                cell.push(&currency).expect("Failed to push currency");
            }
        })
    }

    /// Get allowed currencies
    pub fn get_allowed_currencies() -> Vec<String> {
        ALLOWED_CURRENCIES
            .with_borrow(|cell| cell.iter().map(|currency| currency.to_string()).collect())
    }

    /// Set ecdsa key
    pub fn set_ecdsa_key(ecdsa_key: EcdsaKey) -> DeferredMinterResult<()> {
        ECDSA_KEY.with_borrow_mut(|cell| {
            cell.set(ecdsa_key as u8)
                .map_err(|_| DeferredMinterError::StorageError)
        })?;

        Ok(())
    }

    /// Get ecdsa key
    pub fn get_ecdsa_key() -> EcdsaKey {
        ECDSA_KEY.with_borrow(|cell| EcdsaKey::from(*cell.get()))
    }

    pub fn set_chain_id(chain_id: u64) -> DeferredMinterResult<()> {
        CHAIN_ID.with_borrow_mut(|cell| {
            cell.set(chain_id)
                .map_err(|_| DeferredMinterError::StorageError)
        })?;

        Ok(())
    }

    pub fn get_chain_id() -> u64 {
        CHAIN_ID.with_borrow(|cell| *cell.get())
    }

    pub fn set_evm_rpc(principal: Principal) -> DeferredMinterResult<()> {
        EVM_RPC.with_borrow_mut(|cell| {
            cell.set(principal.into())
                .map_err(|_| DeferredMinterError::StorageError)
        })?;

        Ok(())
    }

    pub fn get_evm_rpc() -> Principal {
        EVM_RPC.with_borrow(|cell| cell.get().0)
    }

    pub fn set_evm_rpc_api(endpoint: String) -> DeferredMinterResult<()> {
        let bytes = endpoint.as_bytes().to_vec();
        EVM_RPC_API.with_borrow_mut(|cell| {
            cell.set(bytes)
                .map_err(|_| DeferredMinterError::StorageError)
        })?;

        Ok(())
    }

    pub fn get_evm_rpc_api() -> Option<String> {
        EVM_RPC_API.with_borrow(|cell| {
            let val = cell.get();
            if val.is_empty() {
                None
            } else {
                Some(String::from_utf8_lossy(val).to_string())
            }
        })
    }

    pub fn set_gas_price(chain_id: u64) -> DeferredMinterResult<()> {
        GAS_PRICE.with_borrow_mut(|cell| {
            cell.set(chain_id)
                .map_err(|_| DeferredMinterError::StorageError)
        })?;

        Ok(())
    }

    pub fn get_gas_price() -> u64 {
        GAS_PRICE.with_borrow(|cell| *cell.get())
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_get_and_set_data_canister() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert_eq!(
            Configuration::get_deferred_data_canister(),
            Principal::anonymous()
        );
        assert!(Configuration::set_deferred_data_canister(principal).is_ok());
        assert_eq!(Configuration::get_deferred_data_canister(), principal);
    }

    #[test]
    fn test_should_get_and_set_evm_rpc() {
        let principal =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();
        assert_eq!(Configuration::get_evm_rpc(), Principal::anonymous());
        assert!(Configuration::set_evm_rpc(principal).is_ok());
        assert_eq!(Configuration::get_evm_rpc(), principal);
    }

    #[test]
    fn test_should_get_and_set_deferred_erc721() {
        let address = H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A").unwrap();
        assert_eq!(Configuration::get_deferred_erc721_contract(), H160::zero());
        assert!(Configuration::set_deferred_erc721_contract(address.clone()).is_ok());
        assert_eq!(Configuration::get_deferred_erc721_contract(), address);
    }

    #[test]
    fn test_should_get_and_set_reward_pool() {
        let address = H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A").unwrap();
        assert_eq!(Configuration::get_reward_pool_contract(), H160::zero());
        assert!(Configuration::set_reward_pool_contract(address.clone()).is_ok());
        assert_eq!(Configuration::get_reward_pool_contract(), address);
    }

    #[test]
    fn test_should_set_and_get_allowed_currencies() {
        assert!(Configuration::get_allowed_currencies().is_empty());
        let currencies = vec!["USD".to_string(), "EUR".to_string(), "GBP".to_string()];
        Configuration::set_allowed_currencies(currencies.clone());
        assert_eq!(Configuration::get_allowed_currencies(), currencies);
    }

    #[test]
    fn test_should_set_and_get_ecdsa_key() {
        assert_eq!(Configuration::get_ecdsa_key(), EcdsaKey::Dfx);
        assert!(Configuration::set_ecdsa_key(EcdsaKey::Production).is_ok());
        assert_eq!(Configuration::get_ecdsa_key(), EcdsaKey::Production);
    }

    #[test]
    fn test_should_set_and_get_chain_id() {
        assert_eq!(Configuration::get_chain_id(), 0);
        assert!(Configuration::set_chain_id(1).is_ok());
        assert_eq!(Configuration::get_chain_id(), 1);
    }

    #[test]
    fn test_should_set_and_get_evm_rpc_api() {
        assert_eq!(Configuration::get_evm_rpc_api(), None);
        assert!(Configuration::set_evm_rpc_api("https://api.ethereum.org".to_string()).is_ok());
        assert_eq!(
            Configuration::get_evm_rpc_api(),
            Some("https://api.ethereum.org".to_string())
        );
    }

    #[test]
    fn test_should_set_and_get_gas_price() {
        assert_eq!(Configuration::get_gas_price(), 20_000_000_000);
        assert!(Configuration::set_gas_price(10_000_000_000).is_ok());
        assert_eq!(Configuration::get_gas_price(), 10_000_000_000);
    }
}
