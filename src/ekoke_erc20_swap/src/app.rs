mod configuration;
mod erc20_bridge;
mod inspect;
mod memory;
mod roles;

use candid::{Nat, Principal};
use did::ekoke::{AllowanceError, Ekoke, EkokeError, EkokeResult};
use did::ekoke_erc20_swap::EkokeErc20SwapInitData;
use did::H160;
use icrc::icrc1::account::Account;
use icrc::IcrcLedgerClient;

use self::configuration::Configuration;
use self::erc20_bridge::Erc20Bridge;
pub use self::inspect::Inspect;
use self::roles::RolesManager;
use crate::utils;

pub struct EkokeErc20SwapCanister;

impl EkokeErc20SwapCanister {
    /// Init ekoke canister
    pub fn init(data: EkokeErc20SwapInitData) {
        // set canisters
        Configuration::set_cketh_ledger_canister(data.cketh_ledger_canister);
        Configuration::set_cketh_minter_canister(data.cketh_minter_canister);
        // Set eth networrk
        Configuration::set_eth_network(data.erc20_network);
        // set ERC20 bridge address
        Configuration::set_erc20_bridge_address(data.erc20_bridge_address);
        // Set ledger canister
        Configuration::set_ledger_canister(data.ledger_id);
        // Set initial swap fee
        Erc20Bridge::set_gas_price(data.erc20_gas_price).unwrap();
        // set roles
        if let Err(err) = RolesManager::set_admins(data.admins) {
            ic_cdk::trap(&format!("Error setting admins: {}", err));
        }
        // set timers
        Self::set_timers();
    }

    pub fn post_upgrade() {
        Self::set_timers();
    }

    /// Set application timers
    fn set_timers() {
        #[cfg(target_family = "wasm")]
        async fn convert_cketh_to_eth_timer() {
            let _ = Erc20Bridge::withdraw_cketh_to_eth().await;
        }

        #[cfg(target_family = "wasm")]
        async fn fetch_ekoke_swapped_events() {
            let _ = Erc20Bridge::swap_erc20_to_icrc().await;
        }

        // convert ckETH to ETH
        #[cfg(target_family = "wasm")]
        ic_cdk_timers::set_timer_interval(crate::constants::CKETH_WITHDRAWAL_INTERVAL, || {
            ic_cdk::spawn(convert_cketh_to_eth_timer());
        });
        // Fetch ERC20 -> ICRC swap events
        #[cfg(target_family = "wasm")]
        ic_cdk_timers::set_timer_interval(
            crate::constants::ERC20_SWAPPED_EVENT_FETCH_INTERVAL,
            || {
                ic_cdk::spawn(fetch_ekoke_swapped_events());
            },
        );
    }

    /// Get the current swap fee.
    ///
    /// If the gas price is older than 3 hours, it refreshes its value
    pub async fn swap_fee() -> EkokeResult<u64> {
        Erc20Bridge::fetch_gas_price().await?;

        Ok(Erc20Bridge::get_swap_fee())
    }

    /// Swap ICRC to ERC20
    pub async fn swap(
        recipient: H160,
        amount: Ekoke,
        from_subaccount: Option<[u8; 32]>,
    ) -> EkokeResult<String> {
        let caller = Account {
            owner: utils::caller(),
            subaccount: from_subaccount,
        };
        let canister_account = utils::account();
        // get current swap fee
        let swap_fee = Erc20Bridge::get_swap_fee();
        // check if caller has given enough ckEth allowance to the canister
        let cketh_client = IcrcLedgerClient::new(Configuration::get_cketh_ledger_canister());
        let cketh_allowance = cketh_client
            .icrc2_allowance(canister_account, caller)
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?;
        if cketh_allowance.allowance < swap_fee {
            return Err(EkokeError::Allowance(AllowanceError::InsufficientFunds));
        }
        if cketh_allowance
            .expires_at
            .map(|expiration| expiration < utils::time())
            .unwrap_or_default()
        {
            return Err(EkokeError::Allowance(AllowanceError::AllowanceExpired));
        }

        // swap
        let txid = Erc20Bridge::swap_icrc_to_erc20(caller, recipient, amount).await?;

        // transfer ckEth to the canister account
        cketh_client
            .icrc2_transfer_from(
                canister_account.subaccount,
                caller,
                canister_account,
                swap_fee.into(),
            )
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))??;

        Ok(txid)
    }

    /// Returns cycles
    pub fn admin_cycles() -> Nat {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        utils::cycles()
    }

    /// Set ckETH ledger canister
    pub fn admin_set_cketh_ledger_canister(canister_id: Principal) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_cketh_ledger_canister(canister_id);
    }

    /// Set ckETH minter canister
    pub fn admin_set_cketh_minter_canister(canister_id: Principal) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_cketh_minter_canister(canister_id);
    }

    /// Set ERC20 bridge address
    pub fn admin_set_erc20_bridge_address(address: H160) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Configuration::set_erc20_bridge_address(address);
    }

    /// Set ERC20 gas price
    pub fn admin_set_erc20_gas_price(gas_price: u64) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Erc20Bridge::set_gas_price(gas_price).unwrap()
    }

    /// Get Ethereum on-chain address for the ekoke canister
    pub async fn admin_eth_wallet_address() -> H160 {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        Erc20Bridge::get_wallet_address().await.unwrap()
    }

    pub fn admin_set_admins(admins: Vec<Principal>) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::set_admins(admins).unwrap();
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr as _;

    use did::ekoke_erc20_swap::EthNetwork;

    use self::utils::caller;
    use super::*;
    use crate::constants::TRANSCRIBE_SWAP_TX_GAS;

    const ERC20_BRIDGE_ADDRESS: &str = "0x2CE04Fd64DB0372F6fb4B7a542f0F9196feE5663";
    const ERC20_GAS_PRICE: u64 = 39_000_000_000;
    const ERC20_SWAP_FEE: u64 = ERC20_GAS_PRICE * TRANSCRIBE_SWAP_TX_GAS;

    #[tokio::test]
    async fn test_should_init_canister() {
        init_canister();

        assert_eq!(RolesManager::get_admins(), vec![caller()]);

        assert_eq!(Configuration::get_cketh_ledger_canister(), caller());
        assert_eq!(Configuration::get_cketh_minter_canister(), caller());
        assert_eq!(
            Configuration::get_erc20_bridge_address(),
            H160::from_hex_str(ERC20_BRIDGE_ADDRESS).unwrap()
        );
        assert_eq!(Erc20Bridge::get_swap_fee(), ERC20_SWAP_FEE);
        assert_eq!(Configuration::get_eth_network(), EthNetwork::Goerli);
    }

    #[tokio::test]
    async fn test_should_swap_icrc_to_erc20() {
        init_canister();

        // set lower gas fee
        EkokeErc20SwapCanister::admin_set_erc20_gas_price(1);
        let amount = 10_000u64;
        let recipient = H160::from_hex_str("0x2CE04Fd64DB0372F6fb4B7a542f0F9196feE5663").unwrap();
        // swap

        assert!(EkokeErc20SwapCanister::swap(recipient, amount.into(), None)
            .await
            .is_ok());
    }

    #[test]
    fn test_should_set_admins() {
        init_canister();
        let admins = vec![Principal::from_str("aaaaa-aa").unwrap()];
        EkokeErc20SwapCanister::admin_set_admins(admins.clone());
        assert_eq!(RolesManager::get_admins(), admins);
    }

    fn init_canister() {
        let data = EkokeErc20SwapInitData {
            admins: vec![caller()],
            cketh_minter_canister: caller(),
            cketh_ledger_canister: caller(),
            erc20_bridge_address: H160::from_hex_str(ERC20_BRIDGE_ADDRESS).unwrap(),
            erc20_gas_price: ERC20_GAS_PRICE,
            erc20_network: EthNetwork::Goerli,
            ledger_id: caller(),
        };
        EkokeErc20SwapCanister::init(data);
    }
}
