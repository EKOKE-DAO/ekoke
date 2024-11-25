use std::cell::RefCell;

use did::deferred::{DeferredMinterError, DeferredMinterResult, EcdsaError, EcdsaKey};
use did::H160;
use ethers_core::k256;
use ethers_core::k256::ecdsa::RecoveryId;
use ethers_core::types::{Bytes, Signature, TransactionRequest, H256};
use ic_cdk::api::management_canister::ecdsa::{
    self, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument, SignWithEcdsaArgument,
};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use secp256k1::PublicKey;
use sha2::Digest as _;
use sha3::Keccak256;

use crate::app::memory::{
    ETH_WALLET_ADDRESS_MEMORY_ID, ETH_WALLET_PUBKEY_MEMORY_ID, MEMORY_MANAGER,
};

const TEST_PUBKEY: &[u8] = &[
    2, 188, 154, 236, 25, 44, 213, 11, 11, 35, 194, 25, 117, 116, 204, 145, 150, 27, 17, 248, 179,
    236, 22, 125, 89, 207, 27, 187, 11, 59, 139, 215, 2,
];

pub struct Wallet {
    chain_id: u64,
    key: EcdsaKey,
}

thread_local! {
    /// Ethereum wallet wallet
    static WALLET_ADDRESS: RefCell<StableCell<H160, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableCell::new(
                MEMORY_MANAGER.with(|mm| mm.get(ETH_WALLET_ADDRESS_MEMORY_ID)),
                H160::zero(),
            ).unwrap()
        );

    /// Ethereum pubkey wallet
    static WALLET_PUBKEY: RefCell<StableCell<Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(
            StableCell::new(
                MEMORY_MANAGER.with(|mm| mm.get(ETH_WALLET_PUBKEY_MEMORY_ID)),
                vec![],
            ).unwrap()
        );
}

impl Wallet {
    pub fn new(key: EcdsaKey, chain_id: u64) -> Self {
        Self { key, chain_id }
    }

    /// Returns the address of the ETH wallet
    pub async fn address(&self) -> DeferredMinterResult<H160> {
        // check if address is already set
        let address = WALLET_ADDRESS.with_borrow(|addr| *addr.get());
        if !address.is_zero() {
            return Ok(address);
        }

        // otherwise calculate if from the pubkey
        let pubkey = self.get_public_key().await?;
        let address = Self::ecdsa_public_key_to_address(&pubkey)?;

        // update address
        WALLET_ADDRESS
            .with_borrow_mut(|addr| addr.set(address))
            .map_err(|_| DeferredMinterError::StorageError)?;

        Ok(address)
    }

    /// Returns the public key of the ETH wallet
    pub async fn get_public_key(&self) -> DeferredMinterResult<Vec<u8>> {
        // check if public key is already set
        let pubkey = WALLET_PUBKEY.with_borrow(|pk| pk.get().clone());
        if !pubkey.is_empty() {
            return Ok(pubkey);
        }

        // otherwise get it from management canister and set public key
        let public_key = self.get_pubkey_from_management_canister().await?;
        WALLET_PUBKEY
            .with_borrow_mut(|pk| pk.set(public_key.clone()))
            .map_err(|_| DeferredMinterError::StorageError)?;

        Ok(public_key)
    }

    /// Signs the transaction with the ETH wallet
    pub async fn sign_transaction(&self, tx: TransactionRequest) -> DeferredMinterResult<Bytes> {
        if cfg!(test) {
            use ethers_signers::{LocalWallet, Signer};

            let wallet = "d8da5b32506763989a81ec84f9430559ebb71d0bc1e2a6e3879e50ffca7b6127"
                .parse::<LocalWallet>()
                .unwrap();
            let signature = wallet.sign_transaction(&tx.clone().into()).await.unwrap();

            return Ok(tx.rlp_signed(&signature));
        }

        let sighash = tx.sighash();
        let (ic_cdk::api::management_canister::ecdsa::SignWithEcdsaResponse { signature },) =
            ecdsa::sign_with_ecdsa(SignWithEcdsaArgument {
                message_hash: sighash.0.to_vec(),
                derivation_path: vec![vec![]],
                key_id: EcdsaKeyId {
                    curve: EcdsaCurve::Secp256k1,
                    name: self.key.to_string(),
                },
            })
            .await
            .map_err(|(code, msg)| DeferredMinterError::CanisterCall(code, msg))?;

        let r = ethers_core::types::U256::from_big_endian(&signature[0..32]);
        let s = ethers_core::types::U256::from_big_endian(&signature[32..64]);

        let public_key = self.get_public_key().await?;

        let v = self.compute_eth_recovery_id(&public_key, sighash, &signature)?;
        let signature = Signature { r, s, v };

        Ok(tx.rlp_signed(&signature))
    }

    /// Returns the public key of the ETH wallet from the management canister
    async fn get_pubkey_from_management_canister(&self) -> DeferredMinterResult<Vec<u8>> {
        if cfg!(test) {
            return Ok(TEST_PUBKEY.to_vec());
        }

        // otherwise get and set it
        let (response,) = ecdsa::ecdsa_public_key(EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path: vec![],
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: self.key.to_string(),
            },
        })
        .await
        .map_err(|(code, msg)| DeferredMinterError::CanisterCall(code, msg))?;

        Ok(response.public_key)
    }

    /// Derive the address of the ETH wallet from the public key SEC1 encoded
    fn ecdsa_public_key_to_address(public_key: &[u8]) -> DeferredMinterResult<H160> {
        let public_key = PublicKey::from_slice(public_key)
            .map_err(|_| DeferredMinterError::Ecdsa(EcdsaError::InvalidPublicKey))?;
        let decompressed_key = public_key.serialize_uncompressed();
        let pub_key_wno_prefix = &decompressed_key[1..];

        // apply keccak256 hash
        let digest = Keccak256::digest(pub_key_wno_prefix);

        let eth_address = &digest[12..];
        Ok(H160::from_slice(eth_address))
    }

    /// Computes the recovery id from the public key, hash and signature
    fn compute_eth_recovery_id(
        &self,
        public_key: &[u8],
        hash: H256,
        signature: &[u8],
    ) -> DeferredMinterResult<u64> {
        let verifying_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(public_key)
            .map_err(|_| DeferredMinterError::Ecdsa(EcdsaError::InvalidPublicKey))?;
        let signature = k256::ecdsa::Signature::from_slice(signature)
            .map_err(|_| DeferredMinterError::Ecdsa(EcdsaError::InvalidSignature))?;
        let recovery_id = k256::ecdsa::RecoveryId::trial_recovery_from_prehash(
            &verifying_key,
            hash.as_bytes(),
            &signature,
        )
        .map(|recid| RecoveryId::new(recid.is_y_odd(), recid.is_x_reduced()))
        .map_err(|_| DeferredMinterError::Ecdsa(EcdsaError::RecoveryIdError))?;

        let v = (recovery_id.to_byte() as u64) + (self.chain_id * 2) + 35;

        Ok(v)
    }
}

#[cfg(test)]
mod test {

    use ethers_core::types::{Transaction, TransactionRequest};
    use ethers_core::utils::rlp::{Decodable, Rlp};
    use ethers_signers::{LocalWallet, Signer};
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn test_should_set_eth_address_and_pubkey() {
        assert!(WALLET_ADDRESS.with_borrow(|addr| addr.get().is_zero()));
        let address = Wallet::new(EcdsaKey::Dfx, 1).address().await.unwrap();
        assert_eq!(
            address,
            H160::from_hex_str("0xc31db061ddd32ad002a1465fde0c92e2cca9c83d").unwrap()
        );
        assert_eq!(
            WALLET_PUBKEY.with_borrow(|pk| pk.get().clone()),
            TEST_PUBKEY,
        );
    }

    #[test]
    fn test_should_derive_address_from_pubkey() {
        let address = Wallet::ecdsa_public_key_to_address(TEST_PUBKEY).unwrap();
        let expected_address =
            H160::from_hex_str("0xc31db061ddd32ad002a1465fde0c92e2cca9c83d").unwrap();

        assert_eq!(address, expected_address);
    }

    #[tokio::test]
    async fn test_should_compute_recovery_id() {
        let wallet = Wallet::new(EcdsaKey::Dfx, 1);

        let from = "d8da5b32506763989a81ec84f9430559ebb71d0bc1e2a6e3879e50ffca7b6127"
            .parse::<LocalWallet>()
            .unwrap()
            .address();

        let tx = TransactionRequest {
            from: Some(from),
            to: Some(
                H160::from_hex_str("0x2CE04Fd64DB0372F6fb4B7a542f0F9196feE5663")
                    .unwrap()
                    .0
                    .into(),
            ),
            value: None,
            nonce: Some(0_u64.into()),
            gas: Some(21000_u64.into()),
            gas_price: Some(1_000_000_000_u64.into()),
            data: None,
            chain_id: Some(1_u64.into()),
        };

        let signed_tx = wallet.sign_transaction(tx.clone().into()).await.unwrap();
        let signed_tx = Transaction::decode(&Rlp::new(&signed_tx)).unwrap();

        let mut signature = vec![0; 64];
        signed_tx.r.to_big_endian(&mut signature[..32]);
        signed_tx.s.to_big_endian(&mut signature[32..64]);

        // now compute recovery id
        let v = Wallet::new(EcdsaKey::Dfx, 1)
            .compute_eth_recovery_id(TEST_PUBKEY, tx.sighash(), &signature.to_vec()[0..64])
            .unwrap();

        assert_eq!(signed_tx.v.as_u64(), v);
    }
}
