//! # ICRC
//!
//! ICRC 1 and 2 traits and types exports

mod client;
pub mod icrc1;
pub mod icrc2;

pub use client::IcrcLedgerClient;
pub use icrc_ledger_types::icrc;
