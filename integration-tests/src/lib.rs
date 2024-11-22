//! # Integration tests
//!
//! This is a test module that is compiled as a separate crate for testing

pub mod actor;
pub mod client;
mod wasm;

use std::io::Read as _;
use std::path::PathBuf;

use candid::{CandidType, Decode, Encode, Principal};
use did::deferred::DeferredDataInitData;
use pocket_ic::common::rest::SubnetConfigSet;
use pocket_ic::{PocketIc, WasmResult};
use serde::de::DeserializeOwned;

use self::wasm::Canister;

const DEFAULT_CYCLES: u128 = 2_000_000_000_000_000;

/// Test environment
pub struct TestEnv {
    pub pic: PocketIc,
    pub deferred_data: Principal,
    pub deferred_minter: Principal,
    pub evm_rpc: Principal,
}

impl TestEnv {
    pub fn query<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: DeserializeOwned + CandidType,
    {
        let result = match self.pic.query_call(canister, caller, method, payload) {
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

    pub fn update<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: DeserializeOwned + CandidType,
    {
        let result = match self.pic.update_call(canister, caller, method, payload) {
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
    pub fn init() -> TestEnv {
        let config = SubnetConfigSet {
            nns: true,
            sns: true,
            application: 1,
            ..Default::default()
        };
        let pic = PocketIc::from_config(config);

        // create canisters
        let deferred_data = pic.create_canister();
        let deferred_minter = pic.create_canister();
        let evm_rpc = pic.create_canister();

        // install
        Self::install_deferred_data(&pic, deferred_data, deferred_minter);

        TestEnv {
            pic,
            deferred_data,
            deferred_minter,
            evm_rpc,
        }
    }

    fn install_deferred_data(pic: &PocketIc, canister_id: Principal, deferred_minter: Principal) {
        pic.add_cycles(canister_id, DEFAULT_CYCLES);
        let wasm_bytes = Self::load_wasm(Canister::DeferredData);

        let init_arg = DeferredDataInitData {
            minter: deferred_minter,
        };
        let init_arg = Encode!(&init_arg).unwrap();

        pic.install_canister(canister_id, wasm_bytes, init_arg, None);
    }

    fn load_wasm(canister: Canister) -> Vec<u8> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(canister.as_path());

        let mut file = std::fs::File::open(path).unwrap();
        let mut wasm_bytes = Vec::new();
        file.read_to_end(&mut wasm_bytes).unwrap();

        wasm_bytes
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        // NOTE: execute test one by one
        for tempdir in std::fs::read_dir(std::path::Path::new("/tmp")).unwrap() {
            let tempdir = tempdir.unwrap();
            if tempdir.file_name().to_string_lossy().starts_with(".tmp") {
                std::fs::remove_dir_all(tempdir.path()).unwrap();
            }
        }
    }
}
