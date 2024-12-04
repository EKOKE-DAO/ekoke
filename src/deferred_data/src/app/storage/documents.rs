use did::deferred::{DataContractError, DeferredDataError, DeferredDataResult};
use did::ID;

use super::{DOCUMENTS, NEXT_DOCUMENT_ID};

pub struct DocumentStorage;

impl DocumentStorage {
    /// Get a document from the storage
    pub fn get_document(id: &ID) -> DeferredDataResult<Vec<u8>> {
        DOCUMENTS.with_borrow(|documents| {
            documents
                .get(&id.clone().into())
                .ok_or(DeferredDataError::Contract(
                    DataContractError::DocumentNotFound(id.clone()),
                ))
        })
    }

    /// Upload a document into the storage.
    ///
    /// Returns the ID of the uploaded document.
    pub fn upload_document(data: Vec<u8>) -> DeferredDataResult<ID> {
        // insert document
        let next_id = Self::next_document_id()?;
        DOCUMENTS.with_borrow_mut(|documents| {
            documents.insert(next_id.clone().into(), data);
        });

        Ok(next_id)
    }

    /// Get next document ID and increment it
    fn next_document_id() -> DeferredDataResult<ID> {
        NEXT_DOCUMENT_ID.with_borrow_mut(|id| {
            let next_id = id.get().0.clone();
            id.set(ID::from(next_id.clone() + 1u64).into())
                .map_err(|_| DeferredDataError::StorageError)?;

            Ok(next_id)
        })
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_document_storage() {
        let data = vec![1, 2, 3, 4, 5];
        let id = DocumentStorage::upload_document(data.clone()).unwrap();
        let stored_data = DocumentStorage::get_document(&id).unwrap();

        assert_eq!(data, stored_data);
        assert_eq!(DocumentStorage::next_document_id().unwrap(), ID::from(1u64));
    }
}
