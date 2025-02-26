use candid::Nat;
use did::deferred::{DeferredDataError, DeferredDataResult, RealEstate, RealEstateError};
use did::ID;

use super::{
    with_real_estate, with_real_estate_mut, with_real_estate_storage_mut, with_real_estates,
};

/// Real estate storage interface
pub struct RealEstateStorage;

impl RealEstateStorage {
    /// Insert a new [`RealEstate`] into the storage.
    ///
    /// Returns the assigned [`ID`]
    pub fn insert(real_estate: RealEstate) -> DeferredDataResult<ID> {
        with_real_estate_storage_mut(|storage| {
            let new_id: Nat = storage.len().into();
            storage.insert(new_id.clone().into(), real_estate);

            Ok(new_id)
        })
    }

    /// Get a [`RealEstate`] by its [`ID`]
    ///
    /// It must return [`RealEstateError::NotFound`] if the [`RealEstate`] is deleted
    pub fn get(id: &ID) -> DeferredDataResult<RealEstate> {
        with_real_estate(id, |real_estate| {
            if real_estate.deleted {
                Err(DeferredDataError::RealEstate(RealEstateError::NotFound(
                    id.clone(),
                )))
            } else {
                Ok(real_estate.clone())
            }
        })
    }

    /// Update a [`RealEstate`] by its [`ID`]
    pub fn update(id: &ID, real_estate: RealEstate) -> DeferredDataResult<()> {
        with_real_estate_mut(id, |real_estate_storage| {
            *real_estate_storage = real_estate;
            Ok(())
        })
    }

    /// Delete a [`RealEstate`] by its [`ID`] marking it as deleted
    pub fn delete(id: &ID) -> DeferredDataResult<()> {
        with_real_estate_mut(id, |real_estate| {
            real_estate.deleted = true;
            Ok(())
        })
    }

    /// get real estate by filter
    pub fn get_real_estates_filter(filter: impl Fn(&RealEstate) -> bool) -> Vec<ID> {
        with_real_estates(|contracts| {
            contracts
                .iter()
                .filter(|(_, contract)| !contract.deleted && filter(contract))
                .map(|(key, _)| key.0.clone())
                .collect()
        })
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::mock_real_estate;

    #[test]
    fn test_should_insert_and_get_property() {
        let real_estate = mock_real_estate();

        let id = RealEstateStorage::insert(real_estate.clone()).unwrap();
        let stored_real_estate = RealEstateStorage::get(&id).unwrap();

        assert_eq!(real_estate, stored_real_estate);
    }

    #[test]
    fn test_should_delete_property() {
        let real_estate = mock_real_estate();

        let id = RealEstateStorage::insert(real_estate.clone()).unwrap();
        RealEstateStorage::delete(&id).unwrap();

        let stored_real_estate = RealEstateStorage::get(&id);

        assert_eq!(
            stored_real_estate,
            Err(DeferredDataError::RealEstate(RealEstateError::NotFound(id)))
        );
    }

    #[test]
    fn test_should_update_property() {
        let real_estate = mock_real_estate();

        let id = RealEstateStorage::insert(real_estate.clone()).unwrap();

        let mut updated_real_estate = real_estate.clone();
        updated_real_estate.name = "Updated name".to_string();

        RealEstateStorage::update(&id, updated_real_estate.clone()).unwrap();

        let stored_real_estate = RealEstateStorage::get(&id).unwrap();

        assert_eq!(updated_real_estate, stored_real_estate);
    }
}
