use std::path::Path;

pub enum Canister {
    Deferred,
    Fly,
    Marketplace,
}

impl Canister {
    pub fn as_path(&self) -> &'static Path {
        match self {
            Canister::Deferred => Path::new("deferred/deferred.wasm"),
            Canister::Fly => Path::new("fly/fly.wasm"),
            Canister::Marketplace => Path::new("marketplace/marketplace.wasm"),
        }
    }
}
