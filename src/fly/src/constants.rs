use std::time::Duration;

/// Token name
pub const ICRC1_NAME: &str = "fly";
/// Token symbol
pub const ICRC1_SYMBOL: &str = "FLY";
/// pico fly
pub const ICRC1_DECIMALS: u8 = 12;
/// Default transfer fee
pub const ICRC1_FEE: u64 = 100_000;
/// Logo
pub const ICRC1_LOGO: &str = "";
/// The ledger will refuse transactions older than this or newer than this
pub const ICRC1_TX_TIME_SKID: Duration = Duration::from_secs(60 * 5);
