use did::deferred::{
    Contract, ContractDocument, ContractDocumentData, DataContractError, DeferredDataError,
    DeferredDataResult, GenericValue, RestrictedProperty,
};
use did::ID;

use super::{
    with_contract, with_contract_mut, with_contracts, with_contracts_mut, DocumentStorage,
};

pub struct ContractStorage;

impl ContractStorage {
    /// Get contract by id
    pub fn get_contract(id: &ID) -> Option<Contract> {
        with_contract(id, |contract| {
            Ok(if contract.closed {
                None
            } else {
                Some(contract.clone())
            })
        })
        .ok()
        .flatten()
    }

    /// Insert contract
    pub fn insert_contract(contract: Contract) {
        with_contracts_mut(|contracts| contracts.insert(contract.id.clone().into(), contract));
    }

    /// Close a contract
    pub fn close_contract(id: &ID) -> DeferredDataResult<()> {
        with_contract_mut(id, |contract| {
            contract.closed = true;
            Ok(())
        })
    }

    /// get contracts
    /// closed contracts are not returned
    pub fn get_contracts() -> Vec<ID> {
        with_contracts(|contracts| {
            contracts
                .iter()
                .filter(|(_, contract)| !contract.closed)
                .map(|(key, _)| key.0.clone())
                .collect()
        })
    }

    /// Update contract property
    pub fn update_contract_property(
        contract_id: &ID,
        key: String,
        value: GenericValue,
    ) -> DeferredDataResult<()> {
        with_contract_mut(contract_id, |contract| {
            let mut found = false;
            for (k, v) in &mut contract.properties {
                if k == &key {
                    *v = value.clone();
                    found = true;
                    break;
                }
            }
            if !found {
                contract.properties.push((key, value));
            }
            Ok(())
        })
    }

    /// Update restricted contract property
    pub fn update_restricted_contract_property(
        contract_id: &ID,
        key: String,
        value: RestrictedProperty,
    ) -> DeferredDataResult<()> {
        with_contract_mut(contract_id, |contract| {
            let mut found = false;
            for (k, v) in &mut contract.restricted_properties {
                if k == &key {
                    *v = value.clone();
                    found = true;
                    break;
                }
            }
            if !found {
                contract.restricted_properties.push((key, value));
            }
            Ok(())
        })
    }

    /// Upload contract document
    pub fn upload_contract_document(
        contract_id: &ID,
        document: ContractDocument,
        data: Vec<u8>,
    ) -> DeferredDataResult<u64> {
        // check if contract exists
        if Self::get_contract(contract_id).is_none() {
            return Err(DeferredDataError::Contract(
                DataContractError::ContractNotFound(contract_id.clone()),
            ));
        }
        // insert document into document storage
        let document_id = DocumentStorage::upload_document(data)?;

        // update contract with document id
        with_contract_mut(contract_id, |contract| {
            contract.documents.push((document_id, document));

            Ok(())
        })?;

        Ok(document_id)
    }

    /// Get contract document
    pub fn get_contract_document(
        contract_id: &ID,
        document_id: u64,
    ) -> DeferredDataResult<ContractDocumentData> {
        // check if contract exists
        let contract = Self::get_contract(contract_id).ok_or_else(|| {
            DeferredDataError::Contract(DataContractError::ContractNotFound(contract_id.clone()))
        })?;

        // check if `document_id` belongs to `contract_id`
        let Some(document_properties) = contract
            .documents
            .iter()
            .find(|(id, _)| *id == document_id)
            .map(|(_, properties)| properties)
        else {
            return Err(DeferredDataError::Contract(
                DataContractError::DocumentNotFound(document_id),
            ));
        };

        // get document from the storage
        let document_data = DocumentStorage::get_document(document_id)?;

        Ok(ContractDocumentData {
            data: document_data,
            mime_type: document_properties.mime_type.clone(),
        })
    }
}

#[cfg(test)]
mod test {

    use did::deferred::{RestrictionLevel, Seller};
    use did::H160;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::with_mock_contract;

    #[test]
    fn test_should_insert_and_get_contract() {
        let seller = vec![Seller {
            address: H160::zero(),
            quota: 100,
        }];

        let contract = with_mock_contract(1, 2, |contract| {
            contract.sellers = seller;
        });

        assert!(ContractStorage::get_contract(&contract.id).is_none());
        ContractStorage::insert_contract(contract.clone());

        assert!(ContractStorage::get_contract(&contract.id).is_some());
    }

    #[test]
    fn test_should_insert_and_get_contract_with_no_buyers() {
        let seller = vec![Seller {
            address: H160::zero(),
            quota: 100,
        }];

        let contract = with_mock_contract(1, 2, |contract| {
            contract.sellers = seller;
            contract.buyers = vec![];
        });

        assert!(ContractStorage::get_contract(&contract.id).is_none());
        ContractStorage::insert_contract(contract.clone());
        assert!(ContractStorage::get_contract(&contract.id).is_some());
        assert_eq!(ContractStorage::get_contracts(), vec![contract.id]);
    }

    #[test]
    fn test_should_update_contract_property() {
        let contract = with_mock_contract(1, 1, |contract| {
            contract.properties.push((
                "contract:address".to_string(),
                GenericValue::TextContent("Rome".to_string()),
            ));
            contract.properties.push((
                "contract:country".to_string(),
                GenericValue::TextContent("Italy".to_string()),
            ));
        });
        ContractStorage::insert_contract(contract);

        assert!(ContractStorage::update_contract_property(
            &1_u64.into(),
            "contract:address".to_string(),
            GenericValue::TextContent("Milan".to_string())
        )
        .is_ok());
        assert_eq!(
            ContractStorage::get_contract(&1_u64.into())
                .unwrap()
                .properties
                .iter()
                .find(|(k, _)| k == "contract:address")
                .unwrap()
                .1,
            GenericValue::TextContent("Milan".to_string())
        );

        assert!(ContractStorage::update_contract_property(
            &1_u64.into(),
            "contract:addressLong".to_string(),
            GenericValue::TextContent("Trieste".to_string())
        )
        .is_ok());
        assert_eq!(
            ContractStorage::get_contract(&1_u64.into())
                .unwrap()
                .properties
                .iter()
                .find(|(k, _)| k == "contract:addressLong")
                .unwrap()
                .1,
            GenericValue::TextContent("Trieste".to_string())
        );
    }

    #[test]
    fn test_should_update_restricted_contract_property() {
        let contract = with_mock_contract(1, 1, |contract| {
            contract.restricted_properties.push((
                "contract:address".to_string(),
                RestrictedProperty {
                    access_list: vec![RestrictionLevel::Seller],
                    value: GenericValue::TextContent("Rome".to_string()),
                },
            ));
        });
        ContractStorage::insert_contract(contract);

        assert!(ContractStorage::update_restricted_contract_property(
            &1_u64.into(),
            "contract:address".to_string(),
            RestrictedProperty {
                access_list: vec![RestrictionLevel::Agent, RestrictionLevel::Seller],
                value: GenericValue::TextContent("Milan".to_string()),
            },
        )
        .is_ok());
        assert_eq!(
            ContractStorage::get_contract(&1_u64.into())
                .unwrap()
                .restricted_properties
                .iter()
                .find(|(k, _)| k == "contract:address")
                .unwrap()
                .1,
            RestrictedProperty {
                access_list: vec![RestrictionLevel::Agent, RestrictionLevel::Seller],
                value: GenericValue::TextContent("Milan".to_string())
            }
        );

        assert!(ContractStorage::update_restricted_contract_property(
            &1_u64.into(),
            "contract:addressLong".to_string(),
            RestrictedProperty {
                access_list: vec![RestrictionLevel::Agent, RestrictionLevel::Seller],
                value: GenericValue::TextContent("Milan".to_string())
            }
        )
        .is_ok());
        assert_eq!(
            ContractStorage::get_contract(&1_u64.into())
                .unwrap()
                .restricted_properties
                .iter()
                .find(|(k, _)| k == "contract:addressLong")
                .unwrap()
                .1,
            RestrictedProperty {
                access_list: vec![RestrictionLevel::Agent, RestrictionLevel::Seller],
                value: GenericValue::TextContent("Milan".to_string())
            }
        );
    }

    #[test]
    fn test_should_close_contract_and_not_return_it() {
        let contract = with_mock_contract(1, 1, |_| {});
        ContractStorage::insert_contract(contract.clone());

        assert!(ContractStorage::get_contract(&contract.id).is_some());
        assert!(ContractStorage::close_contract(&contract.id).is_ok());
        assert!(ContractStorage::get_contract(&contract.id).is_none());
    }

    #[test]
    fn test_should_upload_contract_document() {
        let contract = with_mock_contract(1, 1, |_| {});
        ContractStorage::insert_contract(contract.clone());

        let document = ContractDocument {
            mime_type: "application/pdf".to_string(),
            access_list: vec![RestrictionLevel::Seller],
        };

        let document_id =
            ContractStorage::upload_contract_document(&1_u64.into(), document, vec![1, 2, 3, 4])
                .expect("Failed to upload document");

        // get contract document
        let contract_document = ContractStorage::get_contract_document(&1_u64.into(), document_id)
            .expect("Failed to get contract document");

        assert_eq!(contract_document.mime_type, "application/pdf");
        assert_eq!(contract_document.data, vec![1, 2, 3, 4]);
    }
}
