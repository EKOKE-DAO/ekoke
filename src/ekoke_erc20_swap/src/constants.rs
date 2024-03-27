use std::time::Duration;

/// Duration of THREE hours
pub const THREE_HOURS: Duration = Duration::from_secs(60 * 60 * 3);

/// Ethereum address public key name
#[cfg(target_family = "wasm")]
pub const ETH_PUBKEY_NAME: &str = "key_1";

/// Minimum amount of ckEth which can be withdrawn from the Ethereum bridge (wei)
pub const ETH_MIN_WITHDRAWAL_AMOUNT: u64 = 30_000_000_000_000_000;

#[cfg(target_family = "wasm")]
pub const CKETH_WITHDRAWAL_INTERVAL: Duration = Duration::from_secs(60 * 60 * 24); // 1 day

/// ERC20 Swapped event fetch interval
#[cfg(target_family = "wasm")]
pub const ERC20_SWAPPED_EVENT_FETCH_INTERVAL: Duration = Duration::from_secs(60 * 60); // 1 hour

/// ERC20 EkokeSwapped topic to search in logs
/// Keccak3("EkokeSwapped(address,bytes32,uint256)")
#[cfg(target_family = "wasm")]
pub const ERC20_EKOKE_SWAPPED_TOPIC: &str =
    "0x73237ca1bbcb09a423f8b6dd74772a03e1ceeaefd48bad90b61d01644355eb28";

pub const ERC20_SWAP_FEE_INTEREST: f64 = 1.1; // 10%

/// The gas required to execute a `transcribeSwap` transaction on the ERC20 Ekoke bridge contract
pub const TRANSCRIBE_SWAP_TX_GAS: u64 = 71306;
