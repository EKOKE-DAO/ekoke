//! # Integration tests
//!
//! This is a test module that is compiled as a separate crate for testing

pub mod actor;
pub mod client;
mod wasm;

use std::io::Read as _;
use std::path::PathBuf;
use std::vec;

use candid::{CandidType, Decode, Encode, Nat, Principal};
use did::deferred::DeferredInitData;
use did::fly::{FlyInitData, PicoFly};
use did::marketplace::MarketplaceInitData;
use did::H160;
use pocket_ic::{PocketIc, WasmResult};
use serde::de::DeserializeOwned;

use self::wasm::Canister;
use crate::wasm::Icrc2InitArgs;

const DEFAULT_CYCLES: u128 = 2_000_000_000_000;

/// Test environment
pub struct TestEnv {
    pub pic: PocketIc,
    pub deferred_id: Principal,
    pub fly_id: Principal,
    pub icp_ledger_id: Principal,
    pub marketplace_id: Principal,
    pub xrc_id: Principal,
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
        let pic = PocketIc::new();

        // create canisters
        let icp_ledger_id = pic.create_canister();
        let ckbtc_id = pic.create_canister();
        let cketh_ledger_id = pic.create_canister();
        let cketh_minter_id = pic.create_canister();
        let xrc_id = pic.create_canister();
        let deferred_id = pic.create_canister();
        let fly_id = pic.create_canister();
        let marketplace_id = pic.create_canister();

        // install deferred canister
        Self::install_icrc2(&pic, icp_ledger_id, "ICP", "Internet Computer", 8);
        Self::install_icrc2(&pic, ckbtc_id, "ckBTC", "ckBTC", 8);
        Self::install_icrc2(&pic, cketh_ledger_id, "ckETH", "ckETH", 18);
        // TODO: install ckETH minter
        Self::install_deferred(&pic, deferred_id, fly_id, marketplace_id);
        Self::install_fly(
            &pic,
            fly_id,
            deferred_id,
            marketplace_id,
            xrc_id,
            icp_ledger_id,
            ckbtc_id,
            cketh_ledger_id,
            cketh_minter_id,
        );
        Self::install_marketplace(
            &pic,
            marketplace_id,
            deferred_id,
            fly_id,
            xrc_id,
            icp_ledger_id,
        );
        Self::install_xrc(&pic, xrc_id);

        TestEnv {
            pic,
            deferred_id,
            icp_ledger_id,
            fly_id,
            marketplace_id,
            xrc_id,
        }
    }

    fn install_icrc2(pic: &PocketIc, id: Principal, symbol: &str, name: &str, decimals: u8) {
        pic.add_cycles(id, DEFAULT_CYCLES);
        let wasm_bytes = Self::load_wasm(Canister::Icrc2);
        let init_arg = Encode!(&Icrc2InitArgs {
            name: name.to_string(),
            symbol: symbol.to_string(),
            decimals,
            fee: 10,
            logo: "https://ic0.app/img/logo.png".to_string(),
            minting_account: actor::minting_account(),
            total_supply: Nat::from(21_268_400_889_u64),
            accounts: vec![
                (actor::alice_account(), Nat::from(1_000_000_000)),
                (actor::bob_account(), Nat::from(1_000_000_000)),
            ],
        })
        .unwrap();

        pic.install_canister(id, wasm_bytes, init_arg, None);
    }

    fn install_deferred(
        pic: &PocketIc,
        deferred_id: Principal,
        fly_id: Principal,
        marketplace_id: Principal,
    ) {
        pic.add_cycles(deferred_id, DEFAULT_CYCLES);
        let wasm_bytes = Self::load_wasm(Canister::Deferred);

        let init_arg = DeferredInitData {
            custodians: vec![actor::admin()],
            fly_canister: fly_id,
            marketplace_canister: marketplace_id,
        };
        let init_arg = Encode!(&init_arg).unwrap();

        pic.install_canister(deferred_id, wasm_bytes, init_arg, None);
    }

    #[allow(clippy::too_many_arguments)]
    fn install_fly(
        pic: &PocketIc,
        fly_id: Principal,
        deferred_id: Principal,
        marketplace_id: Principal,
        xrc_canister: Principal,
        icp_ledger_canister: Principal,
        ckbtc_canister: Principal,
        cketh_ledger_canister: Principal,
        cketh_minter_canister: Principal,
    ) {
        pic.add_cycles(fly_id, DEFAULT_CYCLES);
        let wasm_bytes = Self::load_wasm(Canister::Fly);

        let init_arg = FlyInitData {
            admins: vec![actor::admin()],
            total_supply: 8880101010000000000_u64.into(),
            minting_account: actor::minting_account(),
            initial_balances: vec![
                (actor::alice_account(), fly_to_picofly(50_000)),
                (actor::bob_account(), fly_to_picofly(50_000)),
            ],
            deferred_canister: deferred_id,
            marketplace_canister: marketplace_id,
            swap_account: actor::swap_account(),
            xrc_canister,
            icp_ledger_canister,
            ckbtc_canister,
            cketh_ledger_canister,
            cketh_minter_canister,
            erc20_bridge_address: H160::from_hex_str("0x2CE04Fd64DB0372F6fb4B7a542f0F9196feE5663")
                .unwrap(),
        };
        let init_arg = Encode!(&init_arg).unwrap();

        pic.install_canister(fly_id, wasm_bytes, init_arg, None);
    }

    fn install_marketplace(
        pic: &PocketIc,
        marketplace_id: Principal,
        deferred_id: Principal,
        fly_id: Principal,
        xrc_canister: Principal,
        icp_ledger_canister: Principal,
    ) {
        pic.add_cycles(marketplace_id, DEFAULT_CYCLES);
        let wasm_bytes = Self::load_wasm(Canister::Marketplace);

        let init_arg = MarketplaceInitData {
            admins: vec![actor::admin()],
            deferred_canister: deferred_id,
            fly_canister: fly_id,
            xrc_canister,
            icp_ledger_canister,
        };
        let init_arg = Encode!(&init_arg).unwrap();

        pic.install_canister(marketplace_id, wasm_bytes, init_arg, None);
    }

    fn install_xrc(pic: &PocketIc, xrc_id: Principal) {
        pic.add_cycles(xrc_id, DEFAULT_CYCLES);
        let wasm_bytes = Self::load_wasm(Canister::Xrc);
        let init_arg = Encode!(&()).unwrap();

        pic.install_canister(xrc_id, wasm_bytes, init_arg, None);
    }

    fn load_wasm(canister: Canister) -> Vec<u8> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../.dfx/local/canisters");
        path.push(canister.as_path());

        let mut file = std::fs::File::open(path).unwrap();
        let mut wasm_bytes = Vec::new();
        file.read_to_end(&mut wasm_bytes).unwrap();

        wasm_bytes
    }
}

pub fn fly_to_picofly(amount: u64) -> PicoFly {
    let amount = Nat::from(amount);
    let multiplier = Nat::from(1_000_000_000_000_u64);
    amount * multiplier
}
