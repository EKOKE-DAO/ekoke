use std::time::Duration;

/// Token name
pub const ICRC1_NAME: &str = "ekoke";
/// Token symbol
pub const ICRC1_SYMBOL: &str = "EKOKE";
/// pico ekoke
pub const ICRC1_DECIMALS: u8 = 12;
/// Default transfer fee (10.000 picoekoke)
pub const ICRC1_FEE: u64 = 10_000;
/// Logo
pub const ICRC1_LOGO: &str = "";
/// The ledger will refuse transactions older than this or newer than this
pub const ICRC1_TX_TIME_SKID: Duration = Duration::from_secs(60 * 5);

/// Initial "reward multiplier coefficient" value
pub const INITIAL_RMC: f64 = 0.0000042;
/// Minimum reward
pub const MIN_REWARD: u64 = ICRC1_FEE * 2;

/// Factor to multiply the swap fee by
pub const ERC20_SWAP_FEE_MULTIPLIER: f64 = 1.3;
/// Duration of one week
pub const ONE_WEEK: Duration = Duration::from_secs(60 * 60 * 24 * 7);

/// Ethereum address public key name
pub const ETH_PUBKEY_NAME: &str = "eth-pubkey";

#[cfg(target_family = "wasm")]
pub const SPEND_ALLOWANCE_EXPIRED_ALLOWANCE_TIMER_INTERVAL: Duration = ONE_WEEK;

#[cfg(target_family = "wasm")]
pub const LIQUIDITY_POOL_SWAP_INTERVAL: Duration = Duration::from_secs(60 * 60 * 24); // 1 day
