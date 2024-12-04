use std::io::Read as _;
use std::path::PathBuf;

use candid::{CandidType, Decode, Encode, Principal};
use did::deferred::{DeferredDataInitData, DeferredMinterInitData, EcdsaKey};
use ic_log::LogSettingsV2;
use pocket_ic::nonblocking::PocketIc;
use pocket_ic::{PocketIcBuilder, WasmResult};
use serde::de::DeserializeOwned;

use crate::actor::admin;
use crate::client::DeferredMinterClient;
use crate::eth_rpc_client::DeferredErc721Client;
use crate::evm::{Evm, EvmBuilder};
use crate::wasm::Canister;
use crate::TestEnv;

const DEFAULT_CYCLES: u128 = 2_000_000_000_000_000;

/// Test environment
pub struct PocketIcTestEnv {
    pub pic: PocketIc,
    pub deferred_data: Principal,
    pub deferred_minter: Principal,
    pub evm_rpc: Principal,
    pub evm: Evm,
}

impl TestEnv for PocketIcTestEnv {
    fn deferred_data(&self) -> Principal {
        self.deferred_data
    }

    fn deferred_minter(&self) -> Principal {
        self.deferred_minter
    }

    fn evm_rpc(&self) -> Principal {
        self.evm_rpc
    }

    fn evm(&self) -> &Evm {
        &self.evm
    }

    async fn query<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: DeserializeOwned + CandidType,
    {
        let result = match self.pic.query_call(canister, caller, method, payload).await {
            Ok(result) => result,
            Err(e) => anyhow::bail!("Error calling {}: {:?}", method, e),
        };
        let reply = match result {
            WasmResult::Reply(r) => r,
            WasmResult::Reject(r) => anyhow::bail!("{} was rejected: {:?}", method, r),
        };
        let ret_type = Decode!(&reply, R)?;

        Ok(ret_type)
    }

    async fn update<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: DeserializeOwned + CandidType,
    {
        let result = match self
            .pic
            .update_call(canister, caller, method, payload)
            .await
        {
            Ok(result) => result,
            Err(e) => anyhow::bail!("Error calling {}: {:?}", method, e),
        };

        let reply = match result {
            WasmResult::Reply(r) => r,
            WasmResult::Reject(r) => anyhow::bail!("{} was rejected: {:?}", method, r),
        };
        let ret_type = Decode!(&reply, R)?;

        Ok(ret_type)
    }
}

impl PocketIcTestEnv {
    /// Install the canisters needed for the tests
    pub async fn init() -> Self {
        let pic = PocketIcBuilder::new()
            .with_nns_subnet()
            .with_ii_subnet() // To have ECDSA keys
            .with_application_subnet()
            .with_max_request_time_ms(Some(30_000))
            .build_async()
            .await;

        // create canisters
        let deferred_data = pic.create_canister().await;
        println!("Deferred data: {}", deferred_data);
        let deferred_minter = pic.create_canister().await;
        println!("Deferred minter: {}", deferred_minter);
        let evm_rpc = pic.create_canister().await;
        println!("EVM RPC: {}", evm_rpc);

        // install
        let evm = EvmBuilder::setup().await.expect("Failed to setup EVM");

        // install canisters
        Self::install_deferred_data(&pic, deferred_data, deferred_minter).await;
        Self::install_deferred_minter(&pic, deferred_minter, deferred_data, evm_rpc, &evm).await;
        Self::configure_evm_rpc_canister(&pic, evm_rpc).await;

        let env = Self {
            pic,
            deferred_data,
            deferred_minter,
            evm,
            evm_rpc,
        };

        // get eth address and set minter to deferred ERC721
        let minter_address = DeferredMinterClient::new(&env)
            .get_eth_address()
            .await
            .expect("Failed to get eth address");
        println!("Minter address: {}", minter_address);

        DeferredErc721Client::new(&env)
            .admin_set_minter(minter_address)
            .await
            .expect("Failed to set minter");
        DeferredErc721Client::new(&env)
            .admin_set_reward_pool()
            .await
            .expect("Failed to set minter");

        env
    }

    async fn install_deferred_data(
        pic: &PocketIc,
        canister_id: Principal,
        deferred_minter: Principal,
    ) {
        pic.add_cycles(canister_id, DEFAULT_CYCLES).await;
        let wasm_bytes = Self::load_wasm(Canister::DeferredData);

        let init_arg = DeferredDataInitData {
            log_settings: LogSettingsV2 {
                enable_console: true,
                in_memory_records: 128,
                max_record_length: 1024,
                log_filter: "debug".to_string(),
            },
            minter: deferred_minter,
        };
        let init_arg = Encode!(&init_arg).unwrap();

        pic.install_canister(canister_id, wasm_bytes, init_arg, None)
            .await;
    }

    async fn install_deferred_minter(
        pic: &PocketIc,
        canister_id: Principal,
        deferred_data: Principal,
        evm_rpc: Principal,
        evm: &Evm,
    ) {
        pic.add_cycles(canister_id, DEFAULT_CYCLES).await;
        let wasm_bytes = Self::load_wasm(Canister::DeferredMinter);

        let init_args = DeferredMinterInitData {
            allowed_currencies: vec!["USD".to_string()],
            chain_id: evm.chain_id,
            custodians: vec![admin()],
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

        let init_args = Encode!(&init_args).unwrap();

        pic.install_canister(canister_id, wasm_bytes, init_args, None)
            .await;
    }

    async fn configure_evm_rpc_canister(pic: &PocketIc, evm_rpc_canister: Principal) {
        pic.add_cycles(evm_rpc_canister, DEFAULT_CYCLES).await;

        let wasm_bytes = Self::load_wasm(Canister::EvmRpc);

        let init_args = crate::evm::evm_rpc_did::InstallArgs {
            logFilter: None,
            demo: Some(true),
            manageApiKeys: None,
        };

        let init_arg = Encode!(&init_args).unwrap();

        pic.install_canister(evm_rpc_canister, wasm_bytes, init_arg, None)
            .await;
    }

    fn load_wasm(canister: Canister) -> Vec<u8> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(canister.as_path());

        let mut file = std::fs::File::open(path).unwrap();
        let mut wasm_bytes = Vec::new();
        file.read_to_end(&mut wasm_bytes).unwrap();

        wasm_bytes
    }

    pub async fn live(&mut self, live: bool) {
        if live {
            self.pic.make_live(None).await;
        } else {
            self.pic.stop_live().await;
        }
    }
}
