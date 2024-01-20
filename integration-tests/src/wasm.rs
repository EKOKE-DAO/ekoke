use std::path::Path;

use candid::{CandidType, Nat};
use icrc::icrc1::account::Account;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Icrc2InitArgs {
    pub accounts: Vec<(Account, Nat)>,
    pub decimals: u8,
    pub fee: u64,
    pub logo: String,
    pub minting_account: Account,
    pub name: String,
    pub symbol: String,
    pub total_supply: Nat,
}

pub enum Canister {
    Deferred,
    Fly,
    Icrc2,
    Marketplace,
    Xrc,
}

impl Canister {
    pub fn as_path(&self) -> &'static Path {
        match self {
            Canister::Deferred => Path::new("deferred/deferred.wasm"),
            Canister::Fly => Path::new("fly/fly.wasm"),
            Canister::Marketplace => Path::new("marketplace/marketplace.wasm"),
            Canister::Xrc => Path::new("test/xrc.wasm"),
            Canister::Icrc2 => Path::new("test/icrc2-template-canister.wasm"),
        }
    }
}
