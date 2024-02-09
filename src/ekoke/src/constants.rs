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

/// Duration of THREE hours
pub const THREE_HOURS: Duration = Duration::from_secs(60 * 60 * 3);

/// Ethereum address public key name
pub const ETH_PUBKEY_NAME: &str = "eth-pubkey";

/// Minimum amount of ckEth which can be withdrawn from the Ethereum bridge (wei)
pub const ETH_MIN_WITHDRAWAL_AMOUNT: u64 = 30_000_000_000_000_000;

/// The gas required to execute a `transcribeSwap` transaction on the ERC20 Ekoke bridge contract
pub const TRANSCRIBE_SWAP_TX_GAS: u64 = 71306;

#[cfg(target_family = "wasm")]
pub const SPEND_ALLOWANCE_EXPIRED_ALLOWANCE_TIMER_INTERVAL: Duration = ONE_WEEK;

#[cfg(target_family = "wasm")]
pub const LIQUIDITY_POOL_SWAP_INTERVAL: Duration = Duration::from_secs(60 * 60 * 24); // 1 day

#[cfg(target_family = "wasm")]
pub const CKETH_WITHDRAWAL_INTERVAL: Duration = Duration::from_secs(60 * 60 * 24); // 1 day
