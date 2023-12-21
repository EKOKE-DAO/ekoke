/// The number of nanoseconds in a day
pub const NANOSECONDS_IN_A_DAY: u64 = 86_400_000_000_000;

/// The ledger canister id of the ICP token
#[cfg(target_arch = "wasm32")]
pub const ICP_LEDGER_CANISTER: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
