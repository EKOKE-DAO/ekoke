mod configuration;
mod inspect;
mod memory;
mod storage;
#[cfg(test)]
mod test_utils;

use candid::{Nat, Principal};
use did::deferred::{
    Contract, DeferredDataError, DeferredDataInitData, DeferredDataResult, GenericValue,
    RestrictedProperty, RestrictionLevel,
};
use did::ID;
use ethers_core::abi::ethereum_types::H520;
use storage::ContractStorage;

use self::configuration::Configuration;
pub use self::inspect::Inspect;
use crate::utils::{caller, cycles};

/// A message used to verify the ownership of a contract (seller or buyer)
pub struct SignedMessage {
    pub message: String,
    pub signature: H520,
}

pub struct DeferredData;

impl DeferredData {
    pub fn init(data: DeferredDataInitData) {
        Configuration::set_minter(data.minter).expect("Failed to set minter");
        Configuration::set_owner(caller()).expect("Failed to set owner");
    }

    /// Set the minter of the deferred data canister.
    pub fn admin_set_minter(minter: Principal) -> DeferredDataResult<()> {
        if !Inspect::inspect_is_owner(caller()) {
            return Err(DeferredDataError::Unauthorized);
        }

        Configuration::set_minter(minter)
    }

    pub fn admin_cycles() -> Nat {
        if !Inspect::inspect_is_owner(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        cycles()
    }

    /// Insert a contract into the ledger
    pub fn create_contract(contract: Contract) -> DeferredDataResult<()> {
        if !Inspect::inspect_is_minter(caller()) {
            return Err(DeferredDataError::Unauthorized);
        }

        ContractStorage::insert_contract(contract);

        Ok(())
    }

    /// Close a contract
    pub fn close_contract(id: ID) -> DeferredDataResult<()> {
        if !Inspect::inspect_is_minter(caller()) {
            return Err(DeferredDataError::Unauthorized);
        }

        ContractStorage::close_contract(&id)
    }

    /// Get contract data by ID.
    ///
    /// Restricted properties are redacted if the caller is not allowed to access them
    ///
    /// # Arguments
    /// * `id` - The ID of the contract
    /// * `has_proven_ownership` - If the caller has proven ownership of the contract
    pub fn get_contract(id: &ID, signature: Option<SignedMessage>) -> Option<Contract> {
        let mut contract = ContractStorage::get_contract(id)?;
        // redact
        Self::redact_restricted_properties(&mut contract, caller(), signature);

        Some(contract)
    }

    /// Get available contracts
    pub fn get_contracts() -> Vec<ID> {
        ContractStorage::get_contracts()
    }

    /// Update a contract property
    pub fn update_contract_property(
        contract_id: ID,
        key: String,
        value: GenericValue,
    ) -> DeferredDataResult<()> {
        Inspect::inspect_modify_contract(caller(), &contract_id)?;

        ContractStorage::update_contract_property(&contract_id, key, value)
    }

    /// Update a restricted contract property
    pub fn update_restricted_contract_property(
        contract_id: ID,
        key: String,
        value: RestrictedProperty,
    ) -> DeferredDataResult<()> {
        Inspect::inspect_modify_contract(caller(), &contract_id)?;

        ContractStorage::update_restricted_contract_property(&contract_id, key, value)
    }

    /// Redact restricted properties from contract based on access level
    fn redact_restricted_properties(
        contract: &mut Contract,
        caller: Principal,
        signature: Option<SignedMessage>,
    ) {
        let mut redacted_properties = Vec::with_capacity(contract.restricted_properties.len());

        // get caller access level
        let access_level = if contract
            .agency
            .as_ref()
            .map(|agency| agency.owner == caller)
            .unwrap_or_default()
        {
            Some(RestrictionLevel::Agent)
        } else if let Some(signature) = signature {
            Inspect::inspect_signature(&contract.id, signature.signature, signature.message).ok()
        } else {
            None
        };

        // if no access level, redact all
        let Some(access_level) = access_level else {
            // redact all
            contract.restricted_properties = vec![];
            return;
        };

        // otherwise, redact based on our level iterate and check whether we have access
        for (name, property) in contract.restricted_properties.iter() {
            if property.access_list.contains(&access_level) {
                redacted_properties.push((name.clone(), property.clone()));
            }
        }

        contract.restricted_properties = redacted_properties;
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use candid::Nat;
    use did::deferred::RestrictionLevel;
    use did::H160;
    use pretty_assertions::assert_eq;
    use test_utils::{mock_contract, store_mock_contract_with};

    use super::*;

    #[test]
    fn test_should_init() {
        let data = DeferredDataInitData { minter: caller() };

        DeferredData::init(data);

        assert_eq!(Configuration::get_minter(), caller());
    }

    #[test]
    fn test_should_create_contract() {
        init();

        let contract = mock_contract(1, 100);

        DeferredData::create_contract(contract.clone()).expect("Failed to create contract");

        let stored_contract =
            ContractStorage::get_contract(&contract.id).expect("Failed to get contract");
        assert_eq!(contract, stored_contract);
    }

    #[test]
    fn test_should_close_contract() {
        init();

        let contract = mock_contract(1, 100);

        DeferredData::create_contract(contract.clone()).expect("Failed to create contract");

        DeferredData::close_contract(contract.id.clone()).expect("Failed to close contract");

        let stored_contract = ContractStorage::get_contract(&contract.id);
        assert_eq!(stored_contract, None);
    }

    #[test]
    fn test_should_get_contract() {
        init();

        let contract = mock_contract(1, 100);

        DeferredData::create_contract(contract.clone()).expect("Failed to create contract");

        let stored_contract =
            DeferredData::get_contract(&contract.id, None).expect("Failed to get contract");

        assert_eq!(contract, stored_contract);
    }

    #[test]
    fn test_should_set_property() {
        init();

        let contract = mock_contract(1, 100);

        DeferredData::create_contract(contract.clone()).expect("Failed to create contract");

        let key = "key".to_string();
        let value = GenericValue::TextContent("value".to_string());

        DeferredData::update_contract_property(contract.id.clone(), key.clone(), value.clone())
            .expect("Failed to update property");

        let stored_contract =
            ContractStorage::get_contract(&contract.id).expect("Failed to get contract");

        let find = stored_contract
            .properties
            .iter()
            .find(|(k, _)| k == &key)
            .unwrap();

        assert_eq!(find.1, value);
    }

    #[test]
    fn test_should_set_restricted_property() {
        init();

        let contract = mock_contract(1, 100);

        DeferredData::create_contract(contract.clone()).expect("Failed to create contract");

        let key = "key".to_string();
        let value = GenericValue::TextContent("value".to_string());

        DeferredData::update_restricted_contract_property(
            contract.id.clone(),
            key.clone(),
            RestrictedProperty {
                value: value.clone(),
                access_list: vec![RestrictionLevel::Seller],
            },
        )
        .expect("Failed to update property");

        let stored_contract =
            ContractStorage::get_contract(&contract.id).expect("Failed to get contract");

        let find = stored_contract
            .properties
            .iter()
            .find(|(k, _)| k == &key)
            .unwrap();

        assert_eq!(find.1, value);
    }

    #[test]
    fn test_should_redact_properties() {
        let (eth_addr, signature) = signature();
        store_mock_contract_with(1, 100, |contract| {
            contract.buyers = vec![eth_addr];
            contract.restricted_properties.push((
                "contract::restricted1".to_string(),
                RestrictedProperty {
                    value: GenericValue::TextContent("value".to_string()),
                    access_list: vec![RestrictionLevel::Buyer],
                },
            ));
            contract.restricted_properties.push((
                "contract::restricted2".to_string(),
                RestrictedProperty {
                    value: GenericValue::TextContent("value".to_string()),
                    access_list: vec![RestrictionLevel::Seller],
                },
            ));
        });

        // redact
        let contract = DeferredData::get_contract(&Nat::from(1u64), Some(signature)).unwrap();

        assert_eq!(contract.restricted_properties.len(), 1);
    }

    fn signature() -> (H160, SignedMessage) {
        let message = "Hello, Ethereum!".to_string();
        let signature = H520::from_str("0xa5910b58b6345df00a6b9ed66370880b542e55c95e28ea91e3d7981b0e90e63e62bfb46b59c7a423013e0425939b11d027be8d870fced8997aaef914e27c36071b").unwrap();

        (
            H160::from_hex_str("0x5A3b2B9E2056E82fF9f9f7dA3A71053712FaAb50").unwrap(),
            SignedMessage { message, signature },
        )
    }

    fn init() {
        DeferredData::init(DeferredDataInitData { minter: caller() });
    }
}
