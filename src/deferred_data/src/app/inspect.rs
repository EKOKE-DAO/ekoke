use candid::Principal;
use did::deferred::{DataContractError, DeferredDataError, DeferredDataResult, RestrictionLevel};
use did::ID;
use ethers_core::abi::ethereum_types::H520;
use ethers_core::types::{Signature, H160};

use super::configuration::Configuration;
use super::storage::ContractStorage;

pub struct Inspect;

impl Inspect {
    /// Returns true if the caller is the minter.
    pub fn inspect_is_minter(caller: Principal) -> bool {
        caller == Configuration::get_minter()
    }

    /// Returns true if the caller is the owner.
    pub fn inspect_is_owner(caller: Principal) -> bool {
        caller == Configuration::get_owner()
    }

    /// Inspects if the caller is the minter.
    pub fn inspect_modify_contract(caller: Principal, contract: &ID) -> DeferredDataResult<()> {
        if !Inspect::inspect_is_minter(caller) {
            return Err(DeferredDataError::Unauthorized);
        }

        ContractStorage::get_contract(contract).ok_or(DeferredDataError::Contract(
            DataContractError::ContractNotFound(contract.clone()),
        ))?;

        Ok(())
    }

    /// Inspect whether a signature is valid for a contract.
    ///
    /// Returns the restriction level of the caller.
    pub fn inspect_signature(
        contract: &ID,
        signature: H520,
        message: String,
    ) -> DeferredDataResult<RestrictionLevel> {
        let contract = ContractStorage::get_contract(contract).ok_or(
            DeferredDataError::Contract(DataContractError::ContractNotFound(contract.clone())),
        )?;

        // try to get the pubkey from the signature
        let sender_address = Inspect::recover_address(message, signature)?;

        // check if the sender is a buyer or seller
        if contract
            .sellers
            .iter()
            .any(|seller| seller.address.0 == sender_address)
        {
            return Ok(RestrictionLevel::Seller);
        }

        if contract
            .buyers
            .iter()
            .any(|buyer| buyer.0 == sender_address)
        {
            return Ok(RestrictionLevel::Buyer);
        }

        Err(DeferredDataError::Unauthorized)
    }

    /// Recover the public key from a signature
    fn recover_address(message: String, signature: H520) -> DeferredDataResult<H160> {
        let signature = Signature::try_from(signature.as_bytes())
            .map_err(|_| DeferredDataError::InvalidSignature)?;
        let address = signature
            .recover(message)
            .map_err(|_| DeferredDataError::InvalidSignature)?;

        Ok(address)
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use candid::Nat;
    use did::deferred::Seller;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{alice, store_mock_contract, store_mock_contract_with};

    #[test]
    fn test_should_inspect_if_minter() {
        Configuration::set_minter(alice()).expect("Failed to set minter");
        assert_eq!(Inspect::inspect_is_minter(alice()), true);
        assert_eq!(Inspect::inspect_is_minter(Principal::anonymous()), false);
    }

    #[test]
    fn test_should_inspect_if_owner() {
        Configuration::set_owner(alice()).expect("Failed to set owner");
        assert_eq!(Inspect::inspect_is_owner(alice()), true);
        assert_eq!(Inspect::inspect_is_owner(Principal::anonymous()), false);
    }

    #[test]
    fn test_should_inspect_modify_contract() {
        Configuration::set_minter(alice()).expect("Failed to set minter");
        Configuration::set_owner(alice()).expect("Failed to set owner");

        store_mock_contract(1, 60);

        assert_eq!(
            Inspect::inspect_modify_contract(alice(), &Nat::from(1u64)),
            Ok(())
        );
        assert_eq!(
            Inspect::inspect_modify_contract(Principal::anonymous(), &Nat::from(1u64)),
            Err(DeferredDataError::Unauthorized)
        );

        assert_eq!(
            Inspect::inspect_modify_contract(alice(), &Nat::from(2u64)),
            Err(DeferredDataError::Contract(
                DataContractError::ContractNotFound(Nat::from(2u64))
            ))
        );

        store_mock_contract_with(2, 60, |contract| {
            contract.closed = true;
        });

        // not found because the contract is closed
        assert_eq!(
            Inspect::inspect_modify_contract(alice(), &Nat::from(2u64)),
            Err(DeferredDataError::Contract(
                DataContractError::ContractNotFound(Nat::from(2u64))
            ))
        );
    }

    #[test]
    fn test_should_verify_signature_if_seller() {
        // private key is: 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
        let eth_address =
            did::H160::from_hex_str("0x8fd379246834eac74B8419FfdA202CF8051F7A03").unwrap();

        store_mock_contract_with(1, 60, |contract| {
            contract.sellers = vec![Seller {
                address: eth_address,
                quota: 100,
            }]
        });

        let message = "Hello, Ethereum!".to_string();
        let signature = H520::from_str("0x0e9293c16d57e3ea35118a52cc7209871d07db4b74183fbd6758306c2475586a2f64a5837cd7b787bff49e9432aab76de43080b9d98675e8890e16ffc669e6cb1b").unwrap();

        assert_eq!(
            Inspect::inspect_signature(&Nat::from(1u64), signature, message),
            Ok(RestrictionLevel::Seller)
        );
    }

    #[test]
    fn test_should_verify_signature_if_buyer() {
        let eth_address =
            did::H160::from_hex_str("0x8fd379246834eac74B8419FfdA202CF8051F7A03").unwrap();

        store_mock_contract_with(1, 60, |contract| contract.buyers = vec![eth_address]);

        let message = "Hello, Ethereum!".to_string();
        let signature = H520::from_str("0x0e9293c16d57e3ea35118a52cc7209871d07db4b74183fbd6758306c2475586a2f64a5837cd7b787bff49e9432aab76de43080b9d98675e8890e16ffc669e6cb1b").unwrap();

        assert_eq!(
            Inspect::inspect_signature(&Nat::from(1u64), signature, message),
            Ok(RestrictionLevel::Buyer)
        );
    }

    #[test]
    fn test_should_verify_signature_if_invalid() {
        store_mock_contract(1, 60);

        let message = "Hello, Ethereum!".to_string();
        let signature = H520::from_str("0x0e9293c16d57e3ea35118a52cc7209871d07db4b74183fbd6758306c2475586a2f64a5837cd7b787bff49e9432aab76de43080b9d98675e8890e16ffc669e6cb1b").unwrap();

        assert_eq!(
            Inspect::inspect_signature(&Nat::from(1u64), signature, message),
            Err(DeferredDataError::Unauthorized)
        );
    }
}
