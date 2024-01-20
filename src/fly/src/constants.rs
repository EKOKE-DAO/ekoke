use std::time::Duration;

/// Token name
pub const ICRC1_NAME: &str = "fly";
/// Token symbol
pub const ICRC1_SYMBOL: &str = "FLY";
/// pico fly
pub const ICRC1_DECIMALS: u8 = 12;
/// Default transfer fee (10.000 picofly)
pub const ICRC1_FEE: u64 = 10_000;
/// Logo
pub const ICRC1_LOGO: &str = "";
/// The ledger will refuse transactions older than this or newer than this
pub const ICRC1_TX_TIME_SKID: Duration = Duration::from_secs(60 * 5);

/// Initial "reward multiplier coefficient" value
pub const INITIAL_RMC: f64 = 0.0000042;
/// Minimum reward
pub const MIN_REWARD: u64 = ICRC1_FEE * 2;

#[cfg(target_family = "wasm")]
pub const SPEND_ALLOWANCE_EXPIRED_ALLOWANCE_TIMER_INTERVAL: Duration =
    Duration::from_secs(60 * 60 * 24 * 7); // 7 days

#[cfg(target_family = "wasm")]
pub const LIQUIDITY_POOL_SWAP_INTERVAL: Duration = Duration::from_secs(60 * 60 * 24); // 1 day
