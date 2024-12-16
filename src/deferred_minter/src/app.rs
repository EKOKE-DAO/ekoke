use candid::{Nat, Principal};
use contract_id::ContractId;
use data_client::DeferredDataClient;
use did::deferred::{
    Agency, Contract, ContractRegistration, DeferredMinterInitData, DeferredMinterResult, Role,
};
use did::ID;
use ethereum::{DeferredErc721, EvmRpcClient, RewardPool, Wallet};
use ic_log::did::Pagination;
use ic_log::writer::Logs;
use ic_log::{init_log, take_memory_records};

mod agents;
mod configuration;
mod contract_id;
mod data_client;
mod ethereum;
mod inspect;
mod memory;
mod reward;
mod roles;
#[cfg(test)]
pub mod test_utils;

pub(crate) use self::agents::Agents;
use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::reward::Reward;
use self::roles::RolesManager;
use crate::utils::{self, caller};

#[derive(Default)]
/// Deferred minter canister API
pub struct DeferredMinter;

impl DeferredMinter {
    /// Initialize the deferred minter
    pub fn init(init_args: DeferredMinterInitData) {
        Configuration::set_allowed_currencies(init_args.allowed_currencies);
        Configuration::set_deferred_data_canister(init_args.deferred_data)
            .expect("failed to set data canister");
        Configuration::set_deferred_erc721_contract(init_args.deferred_erc721)
            .expect("failed to set erc721 canister");
        Configuration::set_reward_pool_contract(init_args.reward_pool)
            .expect("failed to set reward pool canister");
        Configuration::set_ecdsa_key(init_args.ecdsa_key).expect("failed to set ecdsa key");
        Configuration::set_chain_id(init_args.chain_id).expect("failed to set chain id");
        Configuration::set_evm_rpc(init_args.evm_rpc).expect("failed to set evm rpc");
        if let Some(api_url) = init_args.evm_rpc_api {
            Configuration::set_evm_rpc_api(api_url).expect("failed to set evm rpc api");
        }

        RolesManager::set_custodians(init_args.custodians).expect("failed to set custodians");

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

    /// Get the Ethereum address of the deferred minter
    pub async fn get_eth_address() -> DeferredMinterResult<String> {
        Self::wallet()
            .address()
            .await
            .map(|address| address.to_hex_str())
    }

    /// get agencies
    pub fn get_agencies() -> Vec<Agency> {
        Agents::get_agencies()
    }

    /// Get agency by wallet
    pub fn get_agent(id: Principal) -> Option<Agency> {
        Agents::get_agency_by_wallet(id)
    }

    /// Remove agency by wallet.
    ///
    /// Only a custodian can call this method or the caller must be the owner of the agency
    pub fn remove_agency(wallet: Principal) -> DeferredMinterResult<()> {
        if !Inspect::inspect_remove_agency(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        Agents::remove_agency(wallet);
        // remove role
        RolesManager::remove_role(wallet, Role::Agent)?;

        log::info!("Agency removed: {wallet}",);

        Ok(())
    }

    /// Create a new contract
    pub async fn create_contract(data: ContractRegistration) -> DeferredMinterResult<ID> {
        // inspect
        Inspect::inspect_register_contract(caller(), &data)?;
        // get contract id
        let contract_id = ContractId::get_next_contract_id();
        log::debug!("creating contract with id {contract_id}");

        // create contract
        let token_price = data.token_value;
        let contract = Self::contract_from_registration(contract_id.clone(), data);
        log::debug!("contract data: {contract:?}");

        // get evm rpc client
        let evm_rpc_client = Self::evm_rpc_client();

        // get available reward balance
        let reward_available_balance = Self::reward_pool()
            .available_rewards(&evm_rpc_client)
            .await?;
        log::debug!("reward available balance: {reward_available_balance}");

        // get reward for token
        let token_reward = Reward::get_contract_reward(
            contract.installments,
            reward_available_balance,
            token_price,
        );
        log::debug!("calculated reward for contract {contract_id}: {token_reward:?}");

        // mint contract on erc721
        Self::deferred_erc721()
            .create_contract(
                &Self::wallet(),
                &evm_rpc_client,
                &contract,
                token_reward,
                token_price,
            )
            .await?;
        log::debug!("contract created on Ethereum");

        // insert contract into the storage
        Self::deferred_data().create_contract(contract).await?;
        log::debug!("contract created on data canister");

        // increment contract id
        ContractId::incr_next_contract_id()?;
        log::info!("Contract created with id {contract_id} successfully");

        Ok(contract_id)
    }

    /// Close a contract on both the ERC721 and the data canister
    pub async fn close_contract(contract_id: ID) -> DeferredMinterResult<()> {
        if !Inspect::inspect_is_agent(caller()) && !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        // if we are an agent, we need to check whether we are the agency for the contract
        if RolesManager::is_agent(caller()) {
            log::debug!("caller is an agent");
            let contract = Self::deferred_data().get_contract(&contract_id).await?;
            if contract
                .agency
                .map(|agency| agency.owner != caller())
                .unwrap_or(true)
            {
                log::debug!("caller is not the agency for the contract");
                ic_cdk::trap("Unauthorized");
            }
        }

        // close contract on erc721
        let evm_rpc_client = Self::evm_rpc_client();
        Self::deferred_erc721()
            .close_contract(&Self::wallet(), &evm_rpc_client, contract_id.clone())
            .await?;
        log::debug!("closed contract {contract_id} on Ethereum");

        // close contract on data canister
        Self::deferred_data()
            .close_contract(contract_id.clone())
            .await?;
        log::info!("Contract {contract_id} closed successfully");

        Ok(())
    }

    /// Update allowed currencies
    pub fn admin_set_allowed_currencies(currencies: Vec<String>) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        Configuration::set_allowed_currencies(currencies);
    }

    /// Insert agency into the storage
    pub fn admin_register_agency(wallet: Principal, mut agency: Agency) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        // always set the wallet to the agency
        agency.owner = wallet;
        Agents::insert_agency(wallet, agency);
        // give role to the agent
        if !RolesManager::is_custodian(wallet) {
            RolesManager::give_role(wallet, Role::Agent);
        }

        log::info!("Agency registered: {wallet}",);
    }

    /// Give role to the provied principal
    pub fn admin_set_role(principal: Principal, role: Role) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::give_role(principal, role);

        log::info!("Role {role:?} given to {principal}",);
    }

    /// Remove role from principal.
    ///
    /// Fails if trying to remove the only custodian of the canister
    pub fn admin_remove_role(principal: Principal, role: Role) -> DeferredMinterResult<()> {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        RolesManager::remove_role(principal, role)?;

        log::info!("Role {role:?} removed from {principal}",);

        Ok(())
    }

    /// Set custodians
    pub fn admin_set_custodians(custodians: Vec<Principal>) -> DeferredMinterResult<()> {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        log::info!("Custodians set: {custodians:?}");
        RolesManager::set_custodians(custodians)
    }

    pub fn admin_cycles() -> Nat {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        utils::cycles()
    }

    pub fn admin_ic_logs(pagination: Pagination) -> Logs {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        take_memory_records(pagination.count, pagination.offset)
    }

    /// Set the gas price for the gas station
    pub fn gas_station_set_gas_price(gas_price: u64) -> DeferredMinterResult<()> {
        if !Inspect::inspect_is_gas_station(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        log::info!("Gas price set to {gas_price}");

        Configuration::set_gas_price(gas_price)
    }

    #[inline]
    fn wallet() -> Wallet {
        Wallet::new(
            Configuration::get_ecdsa_key(),
            Configuration::get_chain_id(),
        )
    }

    #[inline]
    fn evm_rpc_client() -> EvmRpcClient {
        EvmRpcClient::new(
            Configuration::get_evm_rpc(),
            Configuration::get_chain_id(),
            Configuration::get_evm_rpc_api(),
        )
    }

    #[inline]
    fn deferred_erc721() -> DeferredErc721 {
        DeferredErc721::from(Configuration::get_deferred_erc721_contract())
    }

    #[inline]
    fn reward_pool() -> RewardPool {
        RewardPool::from(Configuration::get_reward_pool_contract())
    }

    #[inline]
    fn deferred_data() -> DeferredDataClient {
        DeferredDataClient::from(Configuration::get_deferred_data_canister())
    }

    /// Create a contract from the registration data
    fn contract_from_registration(contract_id: ID, data: ContractRegistration) -> Contract {
        // get agency from caller
        let agency = Agents::get_agency_by_wallet(caller());

        Contract {
            id: contract_id,
            r#type: data.r#type,
            sellers: data.sellers,
            buyers: data.buyers,
            installments: data.installments,
            value: data.value,
            deposit: data.deposit,
            currency: data.currency,
            properties: data.properties,
            restricted_properties: data.restricted_properties,
            documents: vec![],
            agency,
            expiration: data.expiration,
            closed: false,
        }
    }
}

#[cfg(test)]
mod test {

    use did::deferred::{Continent, EcdsaKey, Seller};
    use did::H160;
    use ic_log::LogSettingsV2;
    use pretty_assertions::assert_eq;
    use test_utils::{alice, bob};

    use super::*;

    #[tokio::test]
    async fn test_should_init_canister() {
        init();

        assert_eq!(
            Configuration::get_allowed_currencies(),
            vec!["USD".to_string()]
        );
        assert_eq!(Configuration::get_chain_id(), 1);
        assert_eq!(Configuration::get_deferred_data_canister(), alice());
        assert_eq!(
            Configuration::get_deferred_erc721_contract(),
            H160::from_hex_str("0xe57e761aa806c9afe7e06fb0601b17bec310f9c4").unwrap()
        );
        assert_eq!(Configuration::get_ecdsa_key(), EcdsaKey::Dfx);
        assert_eq!(Configuration::get_evm_rpc(), bob());
        assert_eq!(
            Configuration::get_reward_pool_contract(),
            H160::from_hex_str("0x7f4e8e4b4dabf7f5f6e7e7d3f9f5a6e7f6e7f6e7").unwrap()
        );
        assert_eq!(Configuration::get_gas_price(), 20_000_000_000);

        assert!(RolesManager::is_custodian(caller()));
    }

    #[tokio::test]
    #[should_panic]
    async fn test_only_gas_station_should_set_gas_price() {
        init();

        DeferredMinter::admin_set_custodians(vec![alice()]).unwrap();
        DeferredMinter::admin_set_role(caller(), Role::GasStation);
        DeferredMinter::gas_station_set_gas_price(10_000_000_000).unwrap();
    }

    #[tokio::test]
    async fn test_should_set_gas_price() {
        init();

        DeferredMinter::admin_set_role(caller(), Role::GasStation);

        DeferredMinter::gas_station_set_gas_price(10_000_000_000).unwrap();

        assert_eq!(Configuration::get_gas_price(), 10_000_000_000);
    }

    #[tokio::test]
    async fn test_should_set_allowed_currencies() {
        init();

        DeferredMinter::admin_set_allowed_currencies(vec!["EUR".to_string()]);

        assert_eq!(
            Configuration::get_allowed_currencies(),
            vec!["EUR".to_string()]
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn test_only_custodian_should_set_allowed_currencies() {
        init();

        DeferredMinter::admin_set_custodians(vec![alice()]).unwrap();
        DeferredMinter::admin_set_allowed_currencies(vec!["EUR".to_string()]);
    }

    #[tokio::test]
    async fn test_should_register_agency() {
        init();

        DeferredMinter::admin_register_agency(
            bob(),
            Agency {
                name: "Agency".to_string(),
                owner: bob(),
                lat: None,
                lng: None,
                address: String::default(),
                agent: String::default(),
                city: String::default(),
                continent: Continent::Antarctica,
                country: String::default(),
                email: String::default(),
                logo: None,
                mobile: String::default(),
                region: String::default(),
                vat: String::default(),
                website: String::default(),
                zip_code: String::default(),
            },
        );

        let agencies = DeferredMinter::get_agencies();
        assert_eq!(agencies.len(), 1);
        assert_eq!(agencies[0].owner, bob());
    }

    #[tokio::test]
    #[should_panic]
    async fn test_only_custodian_should_register_agency() {
        init();

        DeferredMinter::admin_set_custodians(vec![alice()]).unwrap();
        DeferredMinter::admin_register_agency(
            bob(),
            Agency {
                name: "Agency".to_string(),
                owner: bob(),
                lat: None,
                lng: None,
                address: String::default(),
                agent: String::default(),
                city: String::default(),
                continent: Continent::Antarctica,
                country: String::default(),
                email: String::default(),
                logo: None,
                mobile: String::default(),
                region: String::default(),
                vat: String::default(),
                website: String::default(),
                zip_code: String::default(),
            },
        );
    }

    #[tokio::test]
    async fn test_should_remove_agency() {
        init();

        DeferredMinter::admin_register_agency(
            bob(),
            Agency {
                name: "Agency".to_string(),
                owner: bob(),
                lat: None,
                lng: None,
                address: String::default(),
                agent: String::default(),
                city: String::default(),
                continent: Continent::Antarctica,
                country: String::default(),
                email: String::default(),
                logo: None,
                mobile: String::default(),
                region: String::default(),
                vat: String::default(),
                website: String::default(),
                zip_code: String::default(),
            },
        );

        DeferredMinter::remove_agency(bob()).expect("failed to remove agency");

        let agencies = DeferredMinter::get_agencies();
        assert_eq!(agencies.len(), 0);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_only_custodian_should_remove_agency() {
        init();

        DeferredMinter::admin_register_agency(
            bob(),
            Agency {
                name: "Agency".to_string(),
                owner: bob(),
                lat: None,
                lng: None,
                address: String::default(),
                agent: String::default(),
                city: String::default(),
                continent: Continent::Antarctica,
                country: String::default(),
                email: String::default(),
                logo: None,
                mobile: String::default(),
                region: String::default(),
                vat: String::default(),
                website: String::default(),
                zip_code: String::default(),
            },
        );

        DeferredMinter::admin_set_custodians(vec![alice()]).unwrap();
        DeferredMinter::remove_agency(bob()).expect("failed to remove agency");
    }

    #[tokio::test]
    async fn test_should_create_contract() {
        init();

        let contract = ContractRegistration {
            value: 400_000,
            installments: 400_000 / 100,
            currency: "USD".to_string(),
            buyers: vec![H160::from_hex_str("0x7f4e8e4b4dabf7f5f6e7e7d3f9f5a6e7f6e7f6e7").unwrap()],
            sellers: vec![Seller {
                address: H160::from_hex_str("0x7f4e8e4b4dabf7f5f6e7e7d3f9f5a6e7f6e7f6e7").unwrap(),
                quota: 100,
            }],
            expiration: String::from("2050-01-01"),
            token_value: 100,
            ..Default::default()
        };

        let contract_id = DeferredMinter::create_contract(contract)
            .await
            .expect("failed to create contract");

        assert_eq!(contract_id, 1u64);

        assert_eq!(ContractId::get_next_contract_id(), 2u64);
    }

    #[tokio::test]
    async fn test_should_close_contract() {
        init();

        DeferredMinter::close_contract(1u64.into())
            .await
            .expect("failed to close contract");
    }

    fn init() {
        DeferredMinter::init(DeferredMinterInitData {
            allowed_currencies: vec!["USD".to_string()],
            chain_id: 1,
            custodians: vec![caller()],
            deferred_data: alice(),
            deferred_erc721: H160::from_hex_str("0xe57e761aa806c9afe7e06fb0601b17bec310f9c4")
                .unwrap(),
            ecdsa_key: EcdsaKey::Dfx,
            evm_rpc: bob(),
            evm_rpc_api: None,
            reward_pool: H160::from_hex_str("0x7f4e8e4b4dabf7f5f6e7e7d3f9f5a6e7f6e7f6e7").unwrap(),
            log_settings: LogSettingsV2 {
                enable_console: true,
                log_filter: "debug".to_string(),
                in_memory_records: 128,
                max_record_length: 1000,
            },
        });
    }
}
