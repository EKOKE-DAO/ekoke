use candid::CandidType;
use serde::Deserialize;
use xrc::ExchangeRate;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct InitArgs {
    pub rates: Vec<ExchangeRate>,
}
