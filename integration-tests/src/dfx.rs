use std::io::Read as _;
use std::path::PathBuf;
use std::time::Duration;

use candid::utils::ArgumentEncoder;
use candid::{CandidType, Decode, Principal};
use did::deferred::{DeferredDataInitData, DeferredMinterInitData, EcdsaKey};
use ic_agent::Agent;
use ic_log::LogSettingsV2;
use ic_test_utils::{get_agent, Canister};
use ic_utils::interfaces::ManagementCanister;

use crate::client::DeferredMinterClient;
use crate::eth_rpc_client::DeferredErc721Client;
use crate::evm::{Evm, EvmBuilder};
use crate::wasm::Canister as CanisterType;
use crate::TestEnv;

const DFX_URL: &str = "http://127.0.0.1:4943";
const ADMIN: &str = "admin";
const INIT_CANISTER_CYCLES: u64 = 90_000_000_000_000;

pub struct DfxTestEnv {
    agent: Agent,
    pub deferred_data: Principal,
    pub deferred_minter: Principal,
    pub evm_rpc: Principal,
    pub evm: Evm,
}

impl TestEnv for DfxTestEnv {
    fn deferred_data(&self) -> Principal {
        self.deferred_data
    }

    fn deferred_minter(&self) -> Principal {
        self.deferred_minter
    }

    fn evm(&self) -> &Evm {
        &self.evm
    }

    fn evm_rpc(&self) -> Principal {
        self.evm_rpc
    }

    async fn query<R>(
        &self,
        canister: Principal,
        _caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: serde::de::DeserializeOwned + CandidType,
    {
        let mut call = self.agent.query(&canister, method);
        call.arg = payload;

        let reply = call.call().await?;

        let ret_type = Decode!(&reply, R)?;

        Ok(ret_type)
    }

    async fn update<R>(
        &self,
        canister: Principal,
        _caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: serde::de::DeserializeOwned + CandidType,
    {
        let mut call = self.agent.update(&canister, method);
        call.arg = payload;

        let reply = call.call_and_wait().await?;

        let ret_type = Decode!(&reply, R)?;

        Ok(ret_type)
    }
}

impl DfxTestEnv {
    pub async fn init() -> Self {
        let admin = get_agent(ADMIN, Some(DFX_URL), Some(Duration::from_secs(180)))
            .await
            .expect("Failed to create agent");

        let evm = EvmBuilder::setup().await.expect("Failed to setup evm");

        let deferred_data = Self::create_canister(&admin).await;
        println!("Deferred data canister: {}", deferred_data);
        let deferred_minter = Self::create_canister(&admin).await;
        println!("Deferred minter canister: {}", deferred_minter);
        let evm_rpc = Self::create_canister(&admin).await;
        println!("EVM RPC canister: {}", evm_rpc);

        // install canisters
        Self::install_deferred_data(&admin, deferred_data, deferred_minter).await;
        println!("Deferred data canister installed");
        Self::install_deferred_minter(&admin, deferred_minter, deferred_data, evm_rpc, &evm).await;
        println!("Deferred minter canister installed");
        Self::configure_evm_rpc_canister(&admin, evm_rpc).await;
        println!("EVM RPC canister installed");

        let env = Self {
            agent: admin,
            deferred_data,
            deferred_minter,
            evm_rpc,
            evm,
        };

        // get eth address and construct deferred ERC721
        let minter_address = DeferredMinterClient::new(&env)
            .get_eth_address()
            .await
            .expect("Failed to get eth address");
        println!("Minter address: {minter_address}",);

        DeferredErc721Client::new(&env)
            .admin_set_minter(minter_address)
            .await
            .expect("Failed to set minter");
        DeferredErc721Client::new(&env)
            .admin_set_reward_pool()
            .await
            .expect("Failed to set minter");
        let new_minter_address = DeferredErc721Client::new(&env)
            .get_minter_address()
            .await
            .expect("Failed to get minter address");
        println!("New minter address: {new_minter_address}");
        assert_eq!(
            minter_address, new_minter_address,
            "minter address mismatch"
        );

        println!("DFX test environment initialized");

        env
    }

    pub fn admin(&self) -> Principal {
        self.agent.get_principal().expect("Failed to get principal")
    }

    async fn create_canister(agent: &Agent) -> Principal {
        let wallet = Canister::new_wallet(agent, ADMIN).unwrap();
        wallet
            .create_canister(INIT_CANISTER_CYCLES, None)
            .await
            .expect("Failed to create canister")
    }

    async fn install_deferred_data(
        agent: &Agent,
        canister_id: Principal,
        deferred_minter: Principal,
    ) {
        let wasm_bytes = Self::load_wasm(CanisterType::DeferredData);

        let init_arg = DeferredDataInitData {
            log_settings: LogSettingsV2 {
                enable_console: true,
                in_memory_records: 128,
                max_record_length: 1024,
                log_filter: "debug".to_string(),
            },
            minter: deferred_minter,
        };

        Self::install_canister(agent, canister_id, wasm_bytes, (init_arg,)).await;
    }

    async fn install_deferred_minter(
        agent: &Agent,
        canister_id: Principal,
        deferred_data: Principal,
        evm_rpc: Principal,
        evm: &Evm,
    ) {
        let wasm_bytes = Self::load_wasm(CanisterType::DeferredMinter);

        let init_args = DeferredMinterInitData {
            allowed_currencies: vec!["USD".to_string()],
            chain_id: evm.chain_id,
            custodians: vec![agent.get_principal().expect("Failed to get principal")],
            deferred_data,
            deferred_erc721: evm.deferred,
            ecdsa_key: EcdsaKey::Dfx,
            evm_rpc,
            evm_rpc_api: Some(evm.url.clone()),
            reward_pool: evm.reward_pool,
            log_settings: LogSettingsV2 {
                enable_console: true,
                in_memory_records: 128,
                max_record_length: 1024,
                log_filter: "debug".to_string(),
            },
        };

        Self::install_canister(agent, canister_id, wasm_bytes, (init_args,)).await;
    }

    async fn configure_evm_rpc_canister(agent: &Agent, evm_rpc_canister: Principal) {
        let wasm_bytes = Self::load_wasm(CanisterType::EvmRpc);

        let init_args = crate::evm::evm_rpc_did::InstallArgs {
            logFilter: None,
            demo: Some(true),
            manageApiKeys: None,
        };

        Self::install_canister(agent, evm_rpc_canister, wasm_bytes, (init_args,)).await;
    }

    async fn install_canister(
        agent: &Agent,
        canister: Principal,
        wasm: Vec<u8>,
        args: impl ArgumentEncoder + Send,
    ) {
        let mng = ManagementCanister::create(agent);
        mng.install(&canister, &wasm)
            .with_args(args)
            .call_and_wait()
            .await
            .expect("Failed to install canister");
    }

    fn load_wasm(canister: CanisterType) -> Vec<u8> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(canister.as_path());

        let mut file = std::fs::File::open(path).unwrap();
        let mut wasm_bytes = Vec::new();
        file.read_to_end(&mut wasm_bytes).unwrap();

        wasm_bytes
    }
}
