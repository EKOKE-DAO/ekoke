use std::cell::RefCell;

use did::fly::{EcdsaError, FlyError, FlyResult};
use did::H160;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, StableVec};
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
    static WALLET_PUBKEY: RefCell<StableVec<u8, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableVec::new(
                MEMORY_MANAGER.with(|mm| mm.get(ETH_PUBKEY_MEMORY_ID)),
            ).unwrap()
        );

}

/// Fly canister Ethereum wallet
pub struct EthWallet;

impl EthWallet {
    /// Returns the address of the ETH wallet
    pub async fn address() -> FlyResult<H160> {
        todo!()
    }

    /// Returns the public key of the ETH wallet
    pub async fn get_public_key() -> FlyResult<Vec<u8>> {
        /* <https://github1s.com/dfinity/ic/blob/master/rs/ethereum/cketh/minter/src/state.rs#L355-L367>
            let key_name = read_state(|s| s.ecdsa_key_name.clone());
        log!(DEBUG, "Fetching the ECDSA public key {key_name}");
        let (response,) = ecdsa_public_key(EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path: crate::MAIN_DERIVATION_PATH // vec![]
                .into_iter()
                .map(|x| x.to_vec())
                .collect(),
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name,
            },
        })
         */
        todo!()
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
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

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
