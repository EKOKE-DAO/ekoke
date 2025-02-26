use candid::Principal;
use did::deferred::{
    Contract, ContractError, DeferredDataResult, DeferredMinterError, DeferredMinterResult,
    GenericValue, RealEstate, Seller,
};
use did::{H160, ID};

use crate::utils::caller;

pub struct DeferredDataClient {
    principal: Principal,
}

impl From<Principal> for DeferredDataClient {
    fn from(principal: Principal) -> Self {
        Self { principal }
    }
}

impl DeferredDataClient {
    pub async fn get_contract(&self, contract_id: &ID) -> DeferredMinterResult<Contract> {
        if cfg!(test) {
            return Ok(Contract {
                id: contract_id.clone(),
                r#type: did::deferred::ContractType::Financing,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![
                    H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A").unwrap(),
                ],
                installments: 100,
                value: 250_000,
                deposit: 50_000,
                currency: "EUR".to_string(),
                properties: vec![(
                    "contract:city".to_string(),
                    GenericValue::TextContent("Rome".to_string()),
                )],
                restricted_properties: vec![],
                documents: vec![],
                real_estate: 1u64.into(),
                agency: caller(),
                expiration: "2078-01-01".to_string(),
                closed: false,
            });
        }

        let (contract,) = ic_cdk::call::<_, (Option<Contract>,)>(
            self.principal,
            "get_contract",
            (contract_id.clone(),),
        )
        .await
        .map_err(|(code, err)| did::deferred::DeferredMinterError::CanisterCall(code, err))?;

        contract.ok_or(DeferredMinterError::Contract(
            ContractError::ContractNotFound(contract_id.clone()),
        ))
    }

    /// Create contract on data canister
    pub async fn create_contract(&self, contract: Contract) -> DeferredMinterResult<()> {
        if cfg!(test) {
            return Ok(());
        }

        let (result,) = ic_cdk::call::<_, (DeferredDataResult<()>,)>(
            self.principal,
            "minter_create_contract",
            (contract,),
        )
        .await
        .map_err(|(code, err)| did::deferred::DeferredMinterError::CanisterCall(code, err))?;

        result.map_err(DeferredMinterError::DataCanister)
    }

    /// Close contract on data canister
    pub async fn close_contract(&self, contract_id: ID) -> DeferredMinterResult<()> {
        if cfg!(test) {
            return Ok(());
        }

        let (result,) = ic_cdk::call::<_, (DeferredDataResult<()>,)>(
            self.principal,
            "minter_close_contract",
            (contract_id,),
        )
        .await
        .map_err(|(code, err)| did::deferred::DeferredMinterError::CanisterCall(code, err))?;

        result.map_err(DeferredMinterError::DataCanister)
    }

    pub async fn create_real_estate(&self, real_estate: RealEstate) -> DeferredMinterResult<ID> {
        if cfg!(test) {
            return Ok(1u64.into());
        }

        let (id,) = ic_cdk::call::<_, (DeferredDataResult<ID>,)>(
            self.principal,
            "minter_create_real_estate",
            (real_estate,),
        )
        .await
        .map_err(|(code, err)| did::deferred::DeferredMinterError::CanisterCall(code, err))?;

        id.map_err(DeferredMinterError::DataCanister)
    }

    pub async fn delete_real_estate(&self, id: ID) -> DeferredMinterResult<()> {
        if cfg!(test) {
            return Ok(());
        }

        let (result,) = ic_cdk::call::<_, (DeferredDataResult<()>,)>(
            self.principal,
            "minter_delete_real_estate",
            (id,),
        )
        .await
        .map_err(|(code, err)| did::deferred::DeferredMinterError::CanisterCall(code, err))?;

        result.map_err(DeferredMinterError::DataCanister)
    }

    pub async fn update_real_estate(
        &self,
        id: ID,
        real_estate: RealEstate,
    ) -> DeferredMinterResult<()> {
        if cfg!(test) {
            return Ok(());
        }

        let (result,) = ic_cdk::call::<_, (DeferredDataResult<()>,)>(
            self.principal,
            "minter_update_real_estate",
            (id, real_estate),
        )
        .await
        .map_err(|(code, err)| did::deferred::DeferredMinterError::CanisterCall(code, err))?;

        result.map_err(DeferredMinterError::DataCanister)
    }

    pub async fn get_real_estate(&self, id: ID) -> DeferredMinterResult<RealEstate> {
        if cfg!(test) {
            return Ok(RealEstate {
                deleted: false,
                agency: caller(),
                name: "name".to_string(),
                description: "description".to_string(),
                image: Some("image".to_string()),
                address: Some("address".to_string()),
                country: Some("country".to_string()),
                continent: Some(did::deferred::Continent::Europe),
                region: Some("region".to_string()),
                city: Some("city".to_string()),
                zone: Some("zone".to_string()),
                zip_code: Some("zip_code".to_string()),
                latitude: Some(1.0),
                longitude: Some(2.0),
                square_meters: Some(100),
                rooms: Some(3),
                bathrooms: Some(2),
                bedrooms: Some(1),
                floors: Some(1),
                year_of_construction: Some(2021),
                garden: Some(true),
                balconies: Some(1),
                pool: Some(true),
                garage: Some(true),
                parking: Some(true),
                elevator: Some(true),
                energy_class: Some("A".to_string()),
                youtube: Some("youtube".to_string()),
            });
        }

        let (real_estate,) = ic_cdk::call::<_, (DeferredDataResult<RealEstate>,)>(
            self.principal,
            "get_real_estate",
            (id,),
        )
        .await
        .map_err(|(code, err)| did::deferred::DeferredMinterError::CanisterCall(code, err))?;

        real_estate.map_err(DeferredMinterError::DataCanister)
    }
}
