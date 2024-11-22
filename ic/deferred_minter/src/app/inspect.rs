//! # Inspect
//!
//! Deferred inspect message handler

use candid::Principal;
use did::deferred::{
    ContractError, ContractRegistration, DeferredMinterError, DeferredMinterResult,
};
use did::H160;

use super::agents::Agents;
use super::configuration::Configuration;
use super::roles::RolesManager;

pub struct Inspect;

impl Inspect {
    /// Returns whether caller is custodian of the canister
    pub fn inspect_is_custodian(caller: Principal) -> bool {
        RolesManager::is_custodian(caller)
    }

    /// Returns whether caller is gas station
    pub fn inspect_is_gas_station(caller: Principal) -> bool {
        RolesManager::is_gas_station(caller)
    }

    pub fn inspect_is_agent(caller: Principal) -> bool {
        RolesManager::is_agent(caller)
    }

    /// Inspect register contract parameters:
    ///
    /// - caller must be custodian or agent
    /// - value must be multiple of installments
    /// - must have sellers
    /// - cannot be expired
    /// - currency must be allowed
    pub fn inspect_register_contract(
        caller: Principal,
        data: &ContractRegistration,
    ) -> DeferredMinterResult<()> {
        if !Self::inspect_is_custodian(caller) && !Self::inspect_is_agent(caller) {
            return Err(DeferredMinterError::Unauthorized);
        }

        if data.sellers.is_empty()
            || data
                .sellers
                .iter()
                .any(|seller| seller.address == H160::zero())
        {
            return Err(DeferredMinterError::Contract(
                ContractError::ContractHasNoSeller,
            ));
        }

        if data.buyers.is_empty() || data.buyers.iter().any(|buyer| buyer == &H160::zero()) {
            return Err(DeferredMinterError::Contract(
                ContractError::ContractHasNoBuyer,
            ));
        }

        // verify value must be multiple of installments
        if data.value % data.installments != 0 {
            return Err(DeferredMinterError::Contract(
                ContractError::ContractValueIsNotMultipleOfInstallments,
            ));
        }

        let total_quota = data.sellers.iter().map(|seller| seller.quota).sum::<u8>();
        if total_quota != 100 {
            return Err(DeferredMinterError::Contract(
                ContractError::ContractSellerQuotaIsNot100,
            ));
        }

        // verify expiration date
        let format = time::macros::format_description!("[year]-[month]-[day]");
        match time::Date::parse(&data.expiration, format) {
            Ok(expiration) => {
                if expiration < crate::utils::date() {
                    return Err(DeferredMinterError::Contract(
                        ContractError::BadContractExpiration,
                    ));
                }
            }
            Err(_) => {
                return Err(DeferredMinterError::Contract(
                    ContractError::BadContractExpiration,
                ));
            }
        }

        // verify currency
        let allowed_currencies = Configuration::get_allowed_currencies();
        if !allowed_currencies.contains(&data.currency) {
            return Err(DeferredMinterError::Contract(
                ContractError::CurrencyNotAllowed(data.currency.clone()),
            ));
        }

        Ok(())
    }

    /// Inspect whether caller is custodian or owner of the agency
    pub fn inspect_remove_agency(caller: Principal) -> bool {
        RolesManager::is_custodian(caller) || Agents::get_agency_by_wallet(caller).is_some()
    }
}

#[cfg(test)]
mod test {

    use did::deferred::{Role, Seller};

    use super::*;
    use crate::app::test_utils::{self, bob};
    #[test]
    fn test_should_inspect_whether_to_remove_agency() {
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());

        // register agency
        Agents::insert_agency(bob(), test_utils::mock_agency());
        assert!(Inspect::inspect_remove_agency(caller));
        assert!(Inspect::inspect_remove_agency(bob()));
        assert!(!Inspect::inspect_remove_agency(
            Principal::management_canister()
        ));
    }

    #[test]
    fn test_should_inspect_contract_register_caller_is_not_custodian() {
        // caller is not custodian
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert!(RolesManager::set_custodians(vec![crate::utils::caller()]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                deposit: 50,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_value_is_not_multiple_of_installments() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 110,
                deposit: 50,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_caller_is_not_agent() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        // caller is not agent
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                deposit: 50,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_if_custodian() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                deposit: 50,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_ok());
    }

    #[test]
    fn test_should_inspect_contract_register_if_seller_is_anonymous() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,

                sellers: vec![Seller {
                    address: H160::zero(),
                    quota: 100,
                }],
                deposit: 50,

                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_if_sellers_is_empty() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                sellers: vec![],
                deposit: 50,
                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_if_buyer_is_anonymous() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                deposit: 50,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![H160::zero()],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_if_buyers_is_empty() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                deposit: 50,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_if_quota_is_not_100() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        RolesManager::give_role(caller, Role::Agent);
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                sellers: vec![
                    Seller {
                        address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                            .unwrap(),
                        quota: 20,
                    },
                    Seller {
                        address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7B")
                            .unwrap(),
                        quota: 40,
                    }
                ],
                deposit: 50,
                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_contract_register_if_agent() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        RolesManager::give_role(caller, Role::Agent);
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                deposit: 50,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_ok());
    }

    #[test]
    fn test_should_inspect_contract_register_if_expired() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        RolesManager::give_role(caller, Role::Agent);

        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                deposit: 50,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2018-01-01".to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }

    #[test]
    fn test_should_inspect_invalid_currency() {
        Configuration::set_allowed_currencies(vec![String::from("USD")]);
        let caller = crate::utils::caller();
        assert!(RolesManager::set_custodians(vec![caller]).is_ok());
        assert!(Inspect::inspect_register_contract(
            caller,
            &ContractRegistration {
                value: 100,
                deposit: 50,
                sellers: vec![Seller {
                    address: H160::from_hex_str("0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A")
                        .unwrap(),
                    quota: 100,
                }],
                buyers: vec![
                    H160::from_hex_str("0x6081d7F04a8c31e929f25152d4ad37c83638C62b").unwrap()
                ],
                installments: 25,
                expiration: "2078-01-01".to_string(),
                currency: "EUR".to_string(),
                ..Default::default()
            }
        )
        .is_err());
    }
}
