use candid::{Nat, Principal};
use contract_id::ContractId;
use data_client::DeferredDataClient;
use did::deferred::{
    Agency, Contract, ContractRegistration, DeferredMinterInitData, DeferredMinterResult, Role,
};
use did::ID;
use ethereum::{DeferredErc721, EvmRpcClient, RewardPool, Wallet};

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
mod test_utils;

use self::agents::Agents;
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

    /// Remove agency by wallet.
    ///
    /// Only a custodian can call this method or the caller must be the owner of the agency
    pub fn remove_agency(wallet: Principal) -> DeferredMinterResult<()> {
        if !Inspect::inspect_remove_agency(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        Agents::remove_agency(wallet);
        // remove role
        RolesManager::remove_role(wallet, Role::Agent)
    }

    /// Create a new contract
    pub async fn create_contract(data: ContractRegistration) -> DeferredMinterResult<ID> {
        // inspect
        Inspect::inspect_register_contract(caller(), &data)?;
        // get contract id
        let contract_id = ContractId::get_next_contract_id();

        // create contract
        let contract = Self::contract_from_registration(contract_id.clone(), data);

        // get evm rpc client
        let evm_rpc_client = Self::evm_rpc_client();

        // get available reward balance
        let reward_available_balance = Self::reward_pool()
            .available_rewards(&evm_rpc_client)
            .await?;

        // get reward for token
        let token_reward =
            Reward::get_contract_reward(contract.installments, reward_available_balance as u128);

        // mint contract on erc721
        let token_price = contract.value / contract.installments;
        Self::deferred_erc721()
            .create_contract(
                &Self::wallet(),
                &evm_rpc_client,
                &contract,
                token_reward,
                token_price,
            )
            .await?;

        // insert contract into the storage
        Self::deferred_data().create_contract(contract).await?;

        // increment contract id
        ContractId::incr_next_contract_id()?;

        Ok(contract_id)
    }

    /// Close a contract on both the ERC721 and the data canister
    pub async fn close_contract(contract_id: ID) -> DeferredMinterResult<()> {
        if !Inspect::inspect_is_agent(caller()) && !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        // if we are an agent, we need to check whether we are the agency for the contract
        if RolesManager::is_agent(caller()) {
            let contract = Self::deferred_data().get_contract(&contract_id).await?;
            if contract
                .agency
                .map(|agency| agency.owner != caller())
                .unwrap_or(true)
            {
                ic_cdk::trap("Unauthorized");
            }
        }

        // close contract on erc721
        let evm_rpc_client = Self::evm_rpc_client();
        Self::deferred_erc721()
            .close_contract(&Self::wallet(), &evm_rpc_client, contract_id.clone())
            .await?;

        // close contract on data canister
        Self::deferred_data().close_contract(contract_id).await
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
    }

    /// Give role to the provied principal
    pub fn admin_set_role(principal: Principal, role: Role) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::give_role(principal, role);
    }

    /// Remove role from principal.
    ///
    /// Fails if trying to remove the only custodian of the canister
    pub fn admin_remove_role(principal: Principal, role: Role) -> DeferredMinterResult<()> {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        RolesManager::remove_role(principal, role)
    }

    /// Set custodians
    pub fn admin_set_custodians(custodians: Vec<Principal>) -> DeferredMinterResult<()> {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        RolesManager::set_custodians(custodians)
    }

    pub fn admin_cycles() -> Nat {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        utils::cycles()
    }

    /// Set the gas price for the gas station
    pub fn gas_station_set_gas_price(gas_price: u64) -> DeferredMinterResult<()> {
        if !Inspect::inspect_is_gas_station(caller()) {
            ic_cdk::trap("Unauthorized");
        }

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
            agency,
            expiration: data.expiration,
            closed: false,
        }
    }
}
