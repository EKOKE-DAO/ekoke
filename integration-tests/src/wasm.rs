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
    EkokeIndex,
    EkokeLedger,
    Icrc2,
    Marketplace,
    Xrc,
}

impl Canister {
    pub fn as_path(&self) -> &'static Path {
        match self {
            Canister::Deferred => Path::new("../.dfx/local/canisters/deferred/deferred.wasm"),
            Canister::EkokeIndex => {
                Path::new("../.dfx/local/canisters/ekoke-index/ekoke-index.wasm")
            }
            Canister::EkokeLedger => {
                Path::new("../.dfx/local/canisters/ekoke-ledger/ekoke-ledger.wasm")
            }
            Canister::Marketplace => {
                Path::new("../.dfx/local/canisters/marketplace/marketplace.wasm")
            }
            Canister::Xrc => Path::new("../assets/wasm/xrc-dummy-canister.wasm"),
            Canister::Icrc2 => Path::new("../assets/wasm/icrc2-template-canister.wasm"),
        }
    }
}
