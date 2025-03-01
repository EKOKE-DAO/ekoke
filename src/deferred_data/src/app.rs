mod configuration;
mod inspect;
mod memory;
mod storage;
#[cfg(test)]
pub mod test_utils;

use candid::{Nat, Principal};
use did::deferred::{
    Contract, ContractDocument, ContractDocumentData, DataContractError, DeferredDataError,
    DeferredDataInitData, DeferredDataResult, GenericValue, RealEstate, RestrictedProperty,
    RestrictionLevel,
};
use did::ID;
use ethers_core::abi::ethereum_types::H520;
use ic_log::did::Pagination;
use ic_log::writer::Logs;
use ic_log::{init_log, take_memory_records};

use self::configuration::Configuration;
pub use self::inspect::Inspect;
pub use self::storage::{ContractStorage, RealEstateStorage};
use crate::utils::{caller, cycles};

/// A message used to verify the ownership of a contract (seller or buyer)
pub struct SignedMessage {
    pub message: String,
    pub signature: H520,
}

pub struct DeferredData;

impl DeferredData {
    pub fn init(init_args: DeferredDataInitData) {
        Configuration::set_minter(init_args.minter).expect("Failed to set minter");
        Configuration::set_owner(caller()).expect("Failed to set owner");

        // init logger
        if !cfg!(test) {
            init_log(&init_args.log_settings).expect("failed to init log");
        }
        // set the log settings
        Configuration::set_log_settings(init_args.log_settings)
            .expect("failed to set log settings");
    }

    pub fn post_upgrade() {
        init_log(&Configuration::get_log_settings()).expect("failed to init log");
    }

    /// Set the minter of the deferred data canister.
    pub fn admin_set_minter(minter: Principal) -> DeferredDataResult<()> {
        if !Inspect::inspect_is_owner(caller()) {
            return Err(DeferredDataError::Unauthorized);
        }

        log::info!("Set minter to {minter}");

        Configuration::set_minter(minter)
    }

    pub fn admin_cycles() -> Nat {
        if !Inspect::inspect_is_owner(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        cycles()
    }

    pub fn admin_ic_logs(pagination: Pagination) -> Logs {
        if !Inspect::inspect_is_owner(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        take_memory_records(pagination.count, pagination.offset)
    }

    /// Insert a contract into the ledger
    pub fn create_contract(contract: Contract) -> DeferredDataResult<()> {
        if !Inspect::inspect_is_minter(caller()) {
            return Err(DeferredDataError::Unauthorized);
        }

        let contract_id = contract.id.clone();
        log::debug!("Creating contract {contract_id}");
        ContractStorage::insert_contract(contract);
        log::info!("Contract {contract_id} created");

        Ok(())
    }

    /// Close a contract
    pub fn close_contract(id: ID) -> DeferredDataResult<()> {
        if !Inspect::inspect_is_minter(caller()) {
            return Err(DeferredDataError::Unauthorized);
        }

        log::info!("Closing contract {id}");

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

    /// Upload a contract document
    pub fn upload_contract_document(
        contract_id: ID,
        document: ContractDocument,
        data: Vec<u8>,
    ) -> DeferredDataResult<u64> {
        Inspect::inspect_modify_contract(caller(), &contract_id)?;

        ContractStorage::upload_contract_document(&contract_id, document, data)
    }

    /// Get a contract document
    pub fn get_contract_document(
        contract_id: ID,
        document_id: u64,
        signature: Option<SignedMessage>,
    ) -> DeferredDataResult<ContractDocumentData> {
        // check if we can access document
        let contract = ContractStorage::get_contract(&contract_id).ok_or(
            DeferredDataError::Contract(DataContractError::ContractNotFound(contract_id.clone())),
        )?;

        let document_props = contract
            .documents
            .iter()
            .find(|(id, _)| id == &document_id)
            .map(|(_, props)| props)
            .ok_or(DeferredDataError::Contract(
                DataContractError::DocumentNotFound(document_id),
            ))?;

        // get caller access level
        let access_level = if contract.agency == caller() {
            RestrictionLevel::Agent
        } else if let Some(signature) = signature {
            Inspect::inspect_signature(&contract.id, signature.signature, signature.message)?
        } else {
            RestrictionLevel::Public
        };

        // check if we have access
        if document_props.access_list.contains(&access_level) {
            ContractStorage::get_contract_document(&contract_id, document_id)
        } else {
            Err(DeferredDataError::Unauthorized)
        }
    }

    /// Redact restricted properties from contract based on access level
    fn redact_restricted_properties(
        contract: &mut Contract,
        caller: Principal,
        signature: Option<SignedMessage>,
    ) {
        let mut redacted_properties = Vec::with_capacity(contract.restricted_properties.len());

        // get caller access level
        let access_level = if contract.agency == caller {
            Some(RestrictionLevel::Agent)
        } else if let Some(signature) = signature {
            Inspect::inspect_signature(&contract.id, signature.signature, signature.message).ok()
        } else {
            Some(RestrictionLevel::Public)
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

    /// Create a real estate
    pub fn create_real_estate(real_estate: RealEstate) -> DeferredDataResult<ID> {
        if !Inspect::inspect_is_minter(caller()) {
            return Err(DeferredDataError::Unauthorized);
        }

        log::debug!("creating real estate: {real_estate:?}");
        let id = RealEstateStorage::insert(real_estate)?;
        log::info!("Real estate {id} created");

        Ok(id)
    }

    /// Get a real estate by ID
    pub fn get_real_estate(id: &ID) -> DeferredDataResult<RealEstate> {
        RealEstateStorage::get(id)
    }

    /// Update a real estate by ID
    pub fn update_real_estate(id: &ID, real_estate: RealEstate) -> DeferredDataResult<()> {
        if !Inspect::inspect_is_minter(caller()) {
            return Err(DeferredDataError::Unauthorized);
        }

        log::debug!("Updating real estate {id}");
        RealEstateStorage::update(id, real_estate)?;
        log::info!("Real estate {id} updated");

        Ok(())
    }

    /// Delete a real estate by ID
    pub fn delete_real_estate(id: &ID) -> DeferredDataResult<()> {
        if !Inspect::inspect_is_minter(caller()) {
            return Err(DeferredDataError::Unauthorized);
        }

        log::debug!("Deleting real estate {id}");
        RealEstateStorage::delete(id)?;
        log::info!("Real estate {id} deleted");

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use candid::Nat;
    use did::deferred::{RealEstateError, RestrictionLevel};
    use did::H160;
    use ic_log::LogSettingsV2;
    use pretty_assertions::assert_eq;
    use test_utils::{mock_contract, store_mock_contract_with, with_mock_contract};

    use super::*;
    use crate::app::test_utils::mock_real_estate;

    #[test]
    fn test_should_init() {
        let data = DeferredDataInitData {
            minter: caller(),
            log_settings: Default::default(),
        };

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

        let contract = with_mock_contract(1, 100, |contract| {
            contract.agency = caller();
        });

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

        let contract = with_mock_contract(1, 100, |contract| {
            contract.agency = caller();
        });

        DeferredData::create_contract(contract.clone()).expect("Failed to create contract");

        let key = "key".to_string();
        let value = GenericValue::TextContent("value".to_string());

        DeferredData::update_restricted_contract_property(
            contract.id.clone(),
            key.clone(),
            RestrictedProperty {
                value: value.clone(),
                access_list: vec![RestrictionLevel::Seller, RestrictionLevel::Agent],
            },
        )
        .expect("Failed to update property");

        let stored_contract =
            ContractStorage::get_contract(&contract.id).expect("Failed to get contract");

        let find = stored_contract
            .restricted_properties
            .iter()
            .find(|(k, _)| k == &key)
            .unwrap();

        assert_eq!(find.1.value, value);
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

    #[test]
    fn test_should_create_and_get_real_estate() {
        init();

        let real_estate = mock_real_estate();

        let id = DeferredData::create_real_estate(real_estate.clone()).expect("Failed to create");

        let stored = DeferredData::get_real_estate(&id).expect("Failed to get");

        assert_eq!(real_estate, stored);
    }

    #[test]
    fn test_should_delete_real_estate() {
        init();

        let real_estate = mock_real_estate();

        let id = DeferredData::create_real_estate(real_estate.clone()).expect("Failed to create");

        DeferredData::delete_real_estate(&id).expect("Failed to delete");

        let stored = DeferredData::get_real_estate(&id);

        assert_eq!(
            stored,
            Err(DeferredDataError::RealEstate(RealEstateError::NotFound(id)))
        );
    }

    #[test]
    fn test_should_update_real_estate() {
        init();

        let real_estate = mock_real_estate();

        let id = DeferredData::create_real_estate(real_estate.clone()).expect("Failed to create");

        let mut updated = real_estate.clone();
        updated.address = Some("new address".to_string());

        DeferredData::update_real_estate(&id, updated.clone()).expect("Failed to update");

        let stored = DeferredData::get_real_estate(&id).expect("Failed to get");

        assert_eq!(updated, stored);
    }

    fn signature() -> (H160, SignedMessage) {
        let message = "Hello, Ethereum!".to_string();
        let signature = H520::from_str("0x0e9293c16d57e3ea35118a52cc7209871d07db4b74183fbd6758306c2475586a2f64a5837cd7b787bff49e9432aab76de43080b9d98675e8890e16ffc669e6cb1b").unwrap();

        (
            H160::from_hex_str("0x8fd379246834eac74B8419FfdA202CF8051F7A03").unwrap(),
            SignedMessage { message, signature },
        )
    }

    fn init() {
        DeferredData::init(DeferredDataInitData {
            minter: caller(),
            log_settings: LogSettingsV2 {
                enable_console: true,
                log_filter: "debug".to_string(),
                in_memory_records: 128,
                max_record_length: 1000,
            },
        });
    }
}
