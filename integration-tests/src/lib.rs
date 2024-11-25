//! # Integration tests
//!
//! This is a test module that is compiled as a separate crate for testing

pub mod actor;
pub mod client;
mod dfx;
pub mod eth_rpc_client;
mod evm;
mod pocket_ic;
mod wasm;

use std::future::Future;

use candid::{CandidType, Principal};
use serde::de::DeserializeOwned;

pub use self::dfx::DfxTestEnv;
pub use self::evm::{abi, Evm, WalletName};
pub use self::pocket_ic::PocketIcTestEnv;

pub trait TestEnv {
    fn query<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> impl Future<Output = anyhow::Result<R>>
    where
        R: DeserializeOwned + CandidType;

    fn update<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> impl Future<Output = anyhow::Result<R>>
    where
        R: DeserializeOwned + CandidType;

    fn deferred_data(&self) -> Principal;

    fn deferred_minter(&self) -> Principal;

    fn evm_rpc(&self) -> Principal;

    fn evm(&self) -> &Evm;
}
