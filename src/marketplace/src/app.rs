mod configuration;
mod exchange_rate;
mod inspect;
mod memory;
mod roles;
#[cfg(test)]
mod test_utils;

use candid::{Nat, Principal};
use did::deferred::TokenInfo;
use did::marketplace::{BuyError, MarketplaceError, MarketplaceInitData, MarketplaceResult};
use dip721::TokenIdentifier;
use icrc::icrc1::account::{Account, Subaccount};
use icrc::{icrc2, IcrcLedgerClient};

pub use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::roles::RolesManager;
use crate::app::exchange_rate::ExchangeRate;
use crate::client::{DeferredClient, EkokeClient};
use crate::utils::{caller, cycles, id, time};

struct TokenInfoWithPrice {
    token_info: TokenInfo,
    icp_price_without_interest: u64,
    icp_price_with_interest: u64,
    interest: u64,
    is_caller_contract_buyer: bool,
    is_first_sell: bool,
}

pub struct Marketplace;

impl Marketplace {
    pub fn init(data: MarketplaceInitData) {
        Configuration::set_deferred_canister(data.deferred_canister);
        Configuration::set_ekoke_canister(data.ekoke_canister);
        Configuration::set_xrc_canister(data.xrc_canister);
        Configuration::set_icp_ledger_canister(data.icp_ledger_canister);
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

    pub async fn admin_set_ekoke_canister(canister: Principal) -> MarketplaceResult<()> {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        Configuration::set_ekoke_canister(canister);
        // update liquidity pool canister
        Configuration::update_ekoke_liquidity_pool_account().await?;

        Ok(())
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

    /// Set xrc canister
    pub fn admin_set_xrc_canister(canister_id: Principal) {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_xrc_canister(canister_id);
    }

    /// Set icp ledger canister
    pub fn admin_set_icp_ledger_canister(canister_id: Principal) {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_icp_ledger_canister(canister_id);
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
    /// 1. marketplace verifies that the caller has given Marketplace enough ICP allowance to buy the NFT `icrc2_allowance`
    /// 2. marketplace checks whether the caller is the contract buyer
    ///     2.1 If so, it gets the liquidity pool address for ekoke token (liquity_pool_address)
    ///     2.2 It transfers the `interest_rate` value to the liquidity pool (icrc2_transfer_from)
    /// 3. marketplace calls `icrc2_transfer_from` on icp canister and transfers the ICP price to the previous owner
    /// 4. marketplace calls `transfer_from` on deferred and transfers the NFT to the caller
    /// 5. marketplace checks whether the NFT has been bought for the first time
    ///     5.1 if so, is calls `send_reward` on the ekoke canister passing the caller as the recipient
    /// 6. marketplace checks whether the caller is the contract buyer
    ///     6.1 if so it calls `burn` on deferred passing the token id
    pub async fn buy_token(
        token_id: TokenIdentifier,
        subaccount: Option<Subaccount>,
    ) -> MarketplaceResult<()> {
        let caller_account = Self::caller_account(subaccount);
        let deferred_client = DeferredClient::from(Configuration::get_deferred_canister());
        let ekoke_client = EkokeClient::from(Configuration::get_ekoke_canister());
        // get token info
        let info = Self::get_token_info_with_price(&token_id).await?;
        // 0. checks whether already owns the token
        if Some(caller()) == info.token_info.token.owner {
            return Err(MarketplaceError::Buy(BuyError::CallerAlreadyOwnsToken));
        }
        // 0.1 checks whether token has an owner
        let token_owner = match info.token_info.token.owner {
            Some(owner) => owner,
            None => return Err(MarketplaceError::Buy(BuyError::TokenHasNoOwner)),
        };

        // 1. checks whether caller has given allowance to marketplace to transfer ICP()
        let allowance = Self::get_caller_icp_allowance(caller_account).await?;
        // check whether allowance has expired
        if allowance
            .expires_at
            .map(|expiration| expiration < time())
            .unwrap_or_default()
        {
            return Err(MarketplaceError::Buy(BuyError::IcpAllowanceExpired));
        }

        // check if allowance is enough
        if allowance.allowance < info.icp_price_with_interest {
            return Err(MarketplaceError::Buy(BuyError::IcpAllowanceNotEnough));
        }

        // 2. pay interest to ekoke liquidity pool
        if info.is_caller_contract_buyer {
            Self::top_up_liquidity_pool(caller_account, Nat::from(info.interest)).await?;
        }

        // 3. transfer ICP from caller to token owner
        Self::spend_caller_icp(
            caller_account,
            Account::from(token_owner),
            Nat::from(info.icp_price_without_interest),
        )
        .await?;

        // 4. transfer token from deferred to caller
        deferred_client
            .transfer_from(token_owner, caller(), &token_id)
            .await?;

        // 5. marketplace checks whether the NFT has been buought for the first time
        if info.is_first_sell {
            // call `send_reward` on the ekoke canister passing the caller as the recipient
            ekoke_client
                .send_reward(
                    &info.token_info.contract.id,
                    info.token_info.token.picoekoke_reward,
                    caller_account,
                )
                .await?;
        }

        // 6. marketplace checks whether the caller is the contract buyer
        if info.is_caller_contract_buyer {
            // call `burn` on deferred passing the token id
            deferred_client.burn(&token_id).await?;
        }

        Ok(())
    }

    /// Get token info with price and interest
    async fn get_token_info_with_price(
        token_id: &TokenIdentifier,
    ) -> MarketplaceResult<TokenInfoWithPrice> {
        let token_info = Self::get_token_info(token_id).await?;
        // check if caller is a contract buyer
        let is_caller_contract_buyer = token_info.contract.buyers.contains(&caller());
        // get the price of the token in ICP
        let icp_rate = ExchangeRate::get_rate(
            Configuration::get_xrc_canister(),
            &token_info.contract.currency,
        )
        .await?;
        let icp_price_without_interest = icp_rate.convert(token_info.token.value);

        let icp_price_with_interest = if is_caller_contract_buyer {
            (icp_price_without_interest as f64 * Configuration::get_interest_rate_for_buyer())
                .round() as u64
        } else {
            icp_price_without_interest
        };
        let interest = icp_price_with_interest - icp_price_without_interest;

        Ok(TokenInfoWithPrice {
            is_first_sell: token_info.token.transferred_at.is_none(),
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
        match deferred_client.get_token(token_id).await? {
            Some(token_info) => Ok(token_info),
            None => Err(MarketplaceError::TokenNotFound),
        }
    }

    /// Get ICP token allowance the marketplace can spend for the caller
    async fn get_caller_icp_allowance(
        caller_account: Account,
    ) -> MarketplaceResult<icrc2::allowance::Allowance> {
        let icp_ledger_client = IcrcLedgerClient::from(Configuration::get_icp_ledger_canister());
        icp_ledger_client
            .icrc2_allowance(Self::marketplace_account(), caller_account)
            .await
            .map_err(|(code, msg)| MarketplaceError::CanisterCall(code, msg))
    }

    /// Spend caller allowance in ICP token and send them to the provided recipient
    async fn spend_caller_icp(
        caller_account: Account,
        recipient: Account,
        amount: Nat,
    ) -> MarketplaceResult<Nat> {
        let icp_ledger_client = IcrcLedgerClient::from(Configuration::get_icp_ledger_canister());
        icp_ledger_client
            .icrc2_transfer_from(None, caller_account, recipient, amount)
            .await
            .map_err(|(code, msg)| MarketplaceError::CanisterCall(code, msg))?
            .map_err(MarketplaceError::Icrc2Transfer)
    }

    /// Top up Ekoke canister liquidity pool with the provided amount from the caller account
    async fn top_up_liquidity_pool(caller_account: Account, amount: Nat) -> MarketplaceResult<()> {
        // get liquidity pool account
        let liquidity_pool_account = Configuration::get_ekoke_liquidity_pool_account().await?;
        // transfer interest rate to liquidity from caller
        Self::spend_caller_icp(caller_account, liquidity_pool_account, amount).await?;

        Ok(())
    }

    /// Returns the marketplace account
    #[inline]
    fn marketplace_account() -> Account {
        Account::from(id())
    }

    /// Returns the caller account passing its subaccount
    #[inline]
    fn caller_account(subaccount: Option<Subaccount>) -> Account {
        Account {
            owner: caller(),
            subaccount,
        }
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use super::test_utils::{deferred_canister, ekoke_canister};
    use super::*;
    use crate::utils::caller;

    #[test]
    fn test_should_init_canister() {
        init_canister();
        assert_eq!(Configuration::get_deferred_canister(), deferred_canister());
        assert_eq!(Configuration::get_ekoke_canister(), ekoke_canister());
        assert_eq!(RolesManager::get_admins(), vec![caller()]);

        // check canisters
        assert_eq!(Configuration::get_xrc_canister(), caller());
        assert_eq!(Configuration::get_icp_ledger_canister(), caller());
    }

    #[tokio::test]
    async fn test_should_change_ekoke_canister() {
        init_canister();
        let new_ekoke_canister = Principal::anonymous();
        Marketplace::admin_set_ekoke_canister(new_ekoke_canister)
            .await
            .unwrap();
        assert_eq!(Configuration::get_ekoke_canister(), new_ekoke_canister);
        assert_eq!(
            Configuration::get_ekoke_liquidity_pool_account()
                .await
                .unwrap(),
            Account::from(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap())
        );
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
    fn test_should_set_xrc_canister() {
        init_canister();
        let canister_id = Principal::from_str("aaaaa-aa").unwrap();
        Marketplace::admin_set_xrc_canister(canister_id);
        assert_eq!(Configuration::get_xrc_canister(), canister_id);
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
        let icp_price = Marketplace::get_token_price_icp(TokenIdentifier::from(2_u64))
            .await
            .unwrap();
        assert_eq!(icp_price, 1353013530); // with interest
    }

    #[tokio::test]
    async fn test_should_get_icp_price_without_interest() {
        init_canister();
        let icp_price = Marketplace::get_token_price_icp(TokenIdentifier::from(1_u64))
            .await
            .unwrap();
        assert_eq!(icp_price, 1230012300);
    }

    #[tokio::test]
    async fn test_should_get_token_info_with_price() {
        init_canister();
        let token_info = Marketplace::get_token_info_with_price(&TokenIdentifier::from(1_u64))
            .await
            .unwrap();
        assert_eq!(token_info.token_info.token.id, TokenIdentifier::from(1_u64));
        assert_eq!(1230012300, token_info.icp_price_without_interest);
        assert_eq!(
            token_info.icp_price_with_interest,
            token_info.icp_price_without_interest
        );
        assert_eq!(token_info.interest, 0);
        assert_eq!(token_info.is_first_sell, true);

        let token_info = Marketplace::get_token_info_with_price(&TokenIdentifier::from(2_u64))
            .await
            .unwrap();
        assert_eq!(token_info.token_info.token.id, TokenIdentifier::from(2_u64));
        assert_eq!(1230012300, token_info.icp_price_without_interest);
        assert_eq!(token_info.icp_price_with_interest, 1353013530);
        assert_eq!(
            token_info.interest,
            token_info.icp_price_with_interest - token_info.icp_price_without_interest
        );
        assert_eq!(token_info.is_first_sell, false);
    }

    #[tokio::test]
    async fn test_should_return_caller_icp_allowance() {
        init_canister();
        let allowance = Marketplace::get_caller_icp_allowance(Account::from(caller()))
            .await
            .unwrap();
        assert_eq!(
            allowance,
            icrc2::allowance::Allowance {
                allowance: 5000000000_u64.into(),
                expires_at: None,
            }
        );
    }

    #[tokio::test]
    async fn test_should_not_allow_to_buy_token_already_owned() {
        init_canister();
        assert!(matches!(
            Marketplace::buy_token(TokenIdentifier::from(2_u64), None)
                .await
                .unwrap_err(),
            MarketplaceError::Buy(BuyError::CallerAlreadyOwnsToken)
        ));
    }

    #[tokio::test]
    async fn test_should_not_allow_to_buy_token_without_owner() {
        init_canister();
        assert!(matches!(
            Marketplace::buy_token(TokenIdentifier::from(3_u64), None)
                .await
                .unwrap_err(),
            MarketplaceError::Buy(BuyError::TokenHasNoOwner)
        ));
    }

    #[tokio::test]
    async fn test_should_not_allow_to_buy_if_allowance_is_not_enough() {
        init_canister();
        assert!(matches!(
            Marketplace::buy_token(TokenIdentifier::from(1_u64), Some([1; 32]))
                .await
                .unwrap_err(),
            MarketplaceError::Buy(BuyError::IcpAllowanceNotEnough)
        ));
    }

    #[tokio::test]
    async fn test_should_not_allow_to_buy_if_allowance_is_expired() {
        init_canister();
        assert!(matches!(
            Marketplace::buy_token(TokenIdentifier::from(1_u64), Some([2; 32]))
                .await
                .unwrap_err(),
            MarketplaceError::Buy(BuyError::IcpAllowanceExpired)
        ));
    }

    #[tokio::test]
    async fn test_should_buy_token_if_not_buyer() {
        init_canister();
        assert!(Marketplace::buy_token(TokenIdentifier::from(1_u64), None)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_should_buy_token_if_buyer() {
        init_canister();
        assert!(Marketplace::buy_token(TokenIdentifier::from(4_u64), None)
            .await
            .is_ok());
    }

    #[test]
    fn test_should_set_icp_ledger_canister() {
        init_canister();
        let canister_id = Principal::from_str("aaaaa-aa").unwrap();
        Marketplace::admin_set_icp_ledger_canister(canister_id);
        assert_eq!(Configuration::get_icp_ledger_canister(), canister_id);
    }

    fn init_canister() {
        let data = MarketplaceInitData {
            deferred_canister: deferred_canister(),
            ekoke_canister: ekoke_canister(),
            icp_ledger_canister: caller(),
            admins: vec![caller()],
            xrc_canister: caller(),
        };
        Marketplace::init(data);
    }
}
