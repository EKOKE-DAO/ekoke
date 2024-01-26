use std::cell::RefCell;

use did::fly::{EcdsaError, FlyError, FlyResult};
use did::H160;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use secp256k1::PublicKey;
use sha2::Digest;
use sha3::Keccak256;

use crate::app::memory::{ETH_ADDRESS_MEMORY_ID, ETH_PUBKEY_MEMORY_ID, MEMORY_MANAGER};

thread_local! {

    /// Ethereum wallet wallet
    static WALLET_ADDRESS: RefCell<StableCell<H160, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableCell::new(
                MEMORY_MANAGER.with(|mm| mm.get(ETH_ADDRESS_MEMORY_ID)),
                H160::zero(),
            ).unwrap()
        );

    /// Ethereum pubkey wallet
    static WALLET_PUBKEY: RefCell<StableCell<Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableCell::new(
                MEMORY_MANAGER.with(|mm| mm.get(ETH_PUBKEY_MEMORY_ID)),
                vec![],
            ).unwrap()
        );

}

/// Fly canister Ethereum wallet
pub struct EthWallet;

impl EthWallet {
    /// Returns the address of the ETH wallet
    pub async fn address() -> FlyResult<H160> {
        // check if address is already set
        let address = WALLET_ADDRESS.with_borrow(|addr| addr.get().clone());
        if !address.is_zero() {
            return Ok(address);
        }

        // otherwise calculate if from the pubkey
        let pubkey = Self::get_public_key().await?;
        let address = Self::ecdsa_public_key_to_address(&pubkey)?;

        // update address
        WALLET_ADDRESS.with_borrow_mut(|addr| {
            addr.set(address.clone()).unwrap();
        });

        Ok(address)
    }

    /// Returns the public key of the ETH wallet
    pub async fn get_public_key() -> FlyResult<Vec<u8>> {
        // check if public key is already set
        let pubkey = WALLET_PUBKEY.with_borrow(|pk| pk.get().clone());
        if !pubkey.is_empty() {
            return Ok(pubkey);
        }

        // otherwise get it from management canister and set public key
        let public_key = Self::get_pubkey_from_management_canister().await?;
        WALLET_PUBKEY.with_borrow_mut(|pk| {
            pk.set(public_key.clone()).unwrap();
        });

        Ok(public_key)
    }

    /// Derive the address of the ETH wallet from the public key SEC1 encoded
    fn ecdsa_public_key_to_address(public_key: &[u8]) -> FlyResult<H160> {
        let public_key = PublicKey::from_slice(public_key)
            .map_err(|_| FlyError::Ecdsa(EcdsaError::InvalidPublicKey))?;
        let decompressed_key = public_key.serialize_uncompressed();
        let pub_key_wno_prefix = &decompressed_key[1..];

        // apply keccak256 hash
        let digest = Keccak256::digest(pub_key_wno_prefix);

        let eth_address = &digest[12..];
        Ok(H160::from_slice(eth_address))
    }

    #[cfg(target_family = "wasm")]
    /// Returns the public key of the ETH wallet from the management canister
    async fn get_pubkey_from_management_canister() -> FlyResult<Vec<u8>> {
        use crate::constants::ETH_PUBKEY_NAME;
        use ic_cdk::api::management_canister::ecdsa::{
            self, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
        };
        // otherwise get and set it
        let (response,) = ecdsa::ecdsa_public_key(EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path: vec![vec![]],
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: ETH_PUBKEY_NAME.to_string(),
            },
        })
        .await
        .map_err(|(code, msg)| FlyError::CanisterCall(code, msg))?;

        Ok(response.public_key)
    }

    /// Returns the public key of the ETH wallet from the management canister (test only)
    #[cfg(not(target_family = "wasm"))]
    async fn get_pubkey_from_management_canister() -> FlyResult<Vec<u8>> {
        Ok(vec![
            2, 188, 154, 236, 25, 44, 213, 11, 11, 35, 194, 25, 117, 116, 204, 145, 150, 27, 17,
            248, 179, 236, 22, 125, 89, 207, 27, 187, 11, 59, 139, 215, 2,
        ])
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn test_should_set_eth_address_and_pubkey() {
        assert!(WALLET_ADDRESS.with_borrow(|addr| addr.get().is_zero()));
        let address = EthWallet::address().await.unwrap();
        assert_eq!(
            address,
            H160::from_hex_str("0xc31db061ddd32ad002a1465fde0c92e2cca9c83d").unwrap()
        );
        assert_eq!(
            WALLET_PUBKEY.with_borrow(|pk| pk.get().clone()),
            &[
                2, 188, 154, 236, 25, 44, 213, 11, 11, 35, 194, 25, 117, 116, 204, 145, 150, 27,
                17, 248, 179, 236, 22, 125, 89, 207, 27, 187, 11, 59, 139, 215, 2,
            ]
        );
    }

    #[test]
    fn test_should_derive_address_from_pubkey() {
        let pubkey = &[
            2, 188, 154, 236, 25, 44, 213, 11, 11, 35, 194, 25, 117, 116, 204, 145, 150, 27, 17,
            248, 179, 236, 22, 125, 89, 207, 27, 187, 11, 59, 139, 215, 2,
        ];

        let address = EthWallet::ecdsa_public_key_to_address(pubkey).unwrap();
        let expected_address =
            H160::from_hex_str("0xc31db061ddd32ad002a1465fde0c92e2cca9c83d").unwrap();

        assert_eq!(address, expected_address);
    }
}
