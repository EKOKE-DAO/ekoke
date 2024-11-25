//! # Integration tests
//!
//! This is a test module that is compiled as a separate crate for testing

pub mod actor;
pub mod client;
pub mod eth_rpc_client;
mod evm;
mod wasm;

use std::io::Read as _;
use std::path::PathBuf;

use actor::admin;
use candid::{CandidType, Decode, Encode, Principal};
use did::deferred::{DeferredDataInitData, DeferredMinterInitData, EcdsaKey};
use evm::{Evm, EvmBuilder};
use ic_log::LogSettingsV2;
use pocket_ic::nonblocking::PocketIc;
use pocket_ic::{PocketIcBuilder, WasmResult};
use serde::de::DeserializeOwned;

pub use self::eth_rpc_client::EthRpcClient;
pub use self::evm::{abi, WalletName};
use self::wasm::Canister;

const DEFAULT_CYCLES: u128 = 2_000_000_000_000_000;

/// Test environment
pub struct TestEnv {
    pub pic: PocketIc,
    pub deferred_data: Principal,
    pub deferred_minter: Principal,
    pub evm_rpc: Principal,
    pub evm: Evm,
}

impl TestEnv {
    pub async fn query<R>(
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

    pub async fn update<R>(
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

    /// Install the canisters needed for the tests
    pub async fn init() -> TestEnv {
        let mut pic = PocketIcBuilder::new()
            .with_ii_subnet() // To have ECDSA keys
            .with_application_subnet()
            .build_async()
            .await;

        //let endpoint = pic.make_live(None).await;

        // create canisters
        let deferred_data = pic.create_canister().await;
        println!("Deferred data: {}", deferred_data);
        let deferred_minter = pic.create_canister().await;
        println!("Deferred minter: {}", deferred_minter);
        let evm_rpc = pic.create_canister().await;
        println!("EVM RPC: {}", evm_rpc);

        // install
        let evm = EvmBuilder::setup(&pic, evm_rpc)
            .await
            .expect("Failed to setup EVM");

        // install canisters
        Self::install_deferred_data(&pic, deferred_data, deferred_minter).await;
        Self::install_deferred_minter(&pic, deferred_minter, deferred_data, evm_rpc, &evm).await;

        TestEnv {
            pic,
            deferred_data,
            deferred_minter,
            evm,
            evm_rpc,
        }
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
            evm_rpc_api: Some("http://localhost:3000".to_string()),
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

    pub fn load_wasm(canister: Canister) -> Vec<u8> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(canister.as_path());

        let mut file = std::fs::File::open(path).unwrap();
        let mut wasm_bytes = Vec::new();
        file.read_to_end(&mut wasm_bytes).unwrap();

        wasm_bytes
    }
}
