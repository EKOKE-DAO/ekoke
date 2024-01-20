mod configuration;
mod exchange_rate;
mod inspect;
mod memory;
mod roles;
mod test_utils;

use candid::{Nat, Principal};
use did::deferred::TokenInfo;
use did::marketplace::{MarketplaceError, MarketplaceInitData, MarketplaceResult};
use dip721::TokenIdentifier;

use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::roles::RolesManager;
use crate::app::exchange_rate::ExchangeRate;
use crate::client::{DeferredClient, FlyClient, IcpLedgerClient};
use crate::utils::{caller, cycles};

struct TokenInfoWithPrice {
    token_info: TokenInfo,
    icp_price_without_interest: u64,
    icp_price_with_interest: u64,
    interest: u64,
    is_caller_contract_buyer: bool,
}

pub struct Marketplace;

impl Marketplace {
    pub fn init(data: MarketplaceInitData) {
        Configuration::set_deferred_canister(data.deferred_canister);
        Configuration::set_fly_canister(data.fly_canister);
        RolesManager::set_admins(data.admins).unwrap();
    }

    pub fn admin_cycles() -> Nat {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        cycles()
    }

    /// Sets the admins of the marketplace.
    pub fn admin_set_admins(admins: Vec<Principal>) -> MarketplaceResult<()> {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        RolesManager::set_admins(admins)
    }

    pub fn admin_set_deferred_canister(canister: Principal) {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        Configuration::set_deferred_canister(canister)
    }

    pub fn admin_set_fly_canister(canister: Principal) {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        Configuration::set_fly_canister(canister)
    }

    pub fn admin_set_interest_rate_for_buyer(interest_rate: f64) {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        if interest_rate <= 1.0 {
            ic_cdk::trap("interest rate must be greater than 1.0");
        }
        Configuration::set_interest_rate_for_buyer(interest_rate)
    }

    /// Given a token id, returns the price of the token in ICP.
    pub async fn get_token_price_icp(token_id: TokenIdentifier) -> MarketplaceResult<u64> {
        // get token info
        let token_info = Self::get_token_info_with_price(&token_id).await?;

        Ok(token_info.icp_price_with_interest)
    }

    /// Buy token with the provided id. The buy process, given the IC price, consits of:
    ///
    /// 0. marketplace checks whether the token owner is not the caller
    /// 1. marketplace verifies that the caller has given Marketplace enough ICP allowance to buy the NFT
    /// 2. marketplace checks whether the caller is the contract buyer
    ///     2.1 If so, it gets the liquidity pool address for fly token (liquity_pool_address)
    ///     2.2 It transfers the `interest_rate` value to the liquidity pool (icrc2_transfer_from)
    /// 3. marketplace calls `transfer_from` on deferred and transfers the NFT to the caller
    /// 4. marketplace calls `icrc2_transfer_from` on icp canister and transfers the ICP price to the previous owner
    /// 5. marketplace checks whether the NFT has been buought for the first time
    ///     5.1 if so, is calls `send_reward` on the fly canister passing the caller as the recipient
    /// 6. marketplace checks whether the caller is the contract buyer
    ///     6.1 if so it calls `burn` on deferred passing the token id
    pub async fn buy_token(token_id: TokenIdentifier) -> MarketplaceResult<()> {
        // get token info
        let token_info = Self::get_token_info_with_price(&token_id).await?;

        todo!()
    }

    /// Get token info with price and interest
    async fn get_token_info_with_price(
        token_id: &TokenIdentifier,
    ) -> MarketplaceResult<TokenInfoWithPrice> {
        let token_info = Self::get_token_info(token_id).await?;
        // check if caller is a contract buyer
        let is_caller_contract_buyer = token_info.contract.buyers.contains(&caller());
        println!(
            "{:?}, {:?} is buyer {}",
            token_info.contract.buyers,
            caller(),
            is_caller_contract_buyer
        );
        // get the price of the token in ICP
        let icp_rate = ExchangeRate::get_rate(&token_info.contract.currency).await?;
        let icp_price_without_interest = icp_rate.convert(token_info.token.value);

        let icp_price_with_interest = if is_caller_contract_buyer {
            (icp_price_without_interest as f64 * Configuration::get_interest_rate_for_buyer())
                .round() as u64
        } else {
            icp_price_without_interest
        };
        let interest = icp_price_with_interest - icp_price_without_interest;

        Ok(TokenInfoWithPrice {
            token_info,
            icp_price_without_interest,
            icp_price_with_interest,
            interest,
            is_caller_contract_buyer,
        })
    }

    /// Get token info
    async fn get_token_info(token_id: &TokenIdentifier) -> MarketplaceResult<TokenInfo> {
        // get token info
        let deferred_client = DeferredClient::from(Configuration::get_deferred_canister());
        match deferred_client.get_token(&token_id).await? {
            Some(token_info) => Ok(token_info),
            None => return Err(MarketplaceError::TokenNotFound),
        }
    }
}

#[cfg(test)]
mod test {

    use super::test_utils::{deferred_canister, fly_canister};
    use super::*;
    use crate::utils::caller;

    #[test]
    fn test_should_init_canister() {
        init_canister();
        assert_eq!(Configuration::get_deferred_canister(), deferred_canister());
        assert_eq!(Configuration::get_fly_canister(), fly_canister());
        assert_eq!(RolesManager::get_admins(), vec![caller()]);
    }

    #[test]
    fn test_should_change_fly_canister() {
        init_canister();
        let new_fly_canister = Principal::anonymous();
        Marketplace::admin_set_fly_canister(new_fly_canister);
        assert_eq!(Configuration::get_fly_canister(), new_fly_canister);
    }

    #[test]
    fn test_should_change_deferred_canister() {
        init_canister();
        let new_deferred_canister = Principal::anonymous();
        Marketplace::admin_set_deferred_canister(new_deferred_canister);
        assert_eq!(
            Configuration::get_deferred_canister(),
            new_deferred_canister
        );
    }

    #[test]
    fn test_should_change_interest_rate_for_buyer() {
        init_canister();
        let new_interest_rate = 1.2;
        Marketplace::admin_set_interest_rate_for_buyer(new_interest_rate);
        assert_eq!(
            Configuration::get_interest_rate_for_buyer(),
            new_interest_rate
        );
    }

    #[test]
    #[should_panic]
    fn test_should_not_allow_invalid_interest_rate_value() {
        init_canister();
        let new_interest_rate = 1.0;
        Marketplace::admin_set_interest_rate_for_buyer(new_interest_rate);
    }

    #[tokio::test]
    async fn test_should_get_icp_price_with_interest() {
        init_canister();
        let icp_price = Marketplace::get_token_price_icp(TokenIdentifier::from(2))
            .await
            .unwrap();
        assert_eq!(icp_price, 1353013530); // with interest
    }

    #[tokio::test]
    async fn test_should_get_icp_price_without_interest() {
        init_canister();
        let icp_price = Marketplace::get_token_price_icp(TokenIdentifier::from(1))
            .await
            .unwrap();
        assert_eq!(icp_price, 1230012300);
    }

    #[tokio::test]
    async fn test_should_get_token_info_with_price() {
        init_canister();
        let token_info = Marketplace::get_token_info_with_price(&TokenIdentifier::from(1))
            .await
            .unwrap();
        assert_eq!(token_info.token_info.token.id, TokenIdentifier::from(1));
        assert_eq!(1230012300, token_info.icp_price_without_interest);
        assert_eq!(
            token_info.icp_price_with_interest,
            token_info.icp_price_without_interest
        );
        assert_eq!(token_info.interest, 0);

        let token_info = Marketplace::get_token_info_with_price(&TokenIdentifier::from(2))
            .await
            .unwrap();
        assert_eq!(token_info.token_info.token.id, TokenIdentifier::from(2));
        assert_eq!(1230012300, token_info.icp_price_without_interest);
        assert_eq!(token_info.icp_price_with_interest, 1353013530);
        assert_eq!(
            token_info.interest,
            token_info.icp_price_with_interest - token_info.icp_price_without_interest
        );
    }

    fn init_canister() {
        let data = MarketplaceInitData {
            deferred_canister: deferred_canister(),
            fly_canister: fly_canister(),
            admins: vec![caller()],
        };
        Marketplace::init(data);
    }
}
