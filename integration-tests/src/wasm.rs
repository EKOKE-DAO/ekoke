use std::path::Path;

pub enum Canister {
    DeferredData,
    DeferredMinter,
    EvmRpc,
}

impl Canister {
    pub fn as_path(&self) -> &'static Path {
        match self {
            Canister::DeferredData => {
                Path::new("../.dfx/local/canisters/deferred_data/deferred_data.wasm")
            }
            Canister::DeferredMinter => {
                Path::new("../.dfx/local/canisters/deferred_minter/deferred_minter.wasm")
            }
            Canister::EvmRpc => Path::new("../assets/wasm/evm_rpc.wasm.gz"),
        }
    }
}
