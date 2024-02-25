use std::ops::Div as _;

use candid::Principal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EkokeSwappedRpcResult {
    pub result: Vec<EkokeSwapped>,
}

#[derive(Debug, Deserialize)]
pub struct EkokeSwapped {
    pub data: String,
    pub block_number: String,
    pub topics: Vec<String>,
}

impl EkokeSwapped {
    /// parse 0x prefixed hex string to u64
    #[allow(dead_code)]
    pub fn block_number(&self) -> Result<u64, String> {
        let block_number = self.block_number.trim_start_matches("0x");
        u64::from_str_radix(block_number, 16).map_err(|err| err.to_string())
    }

    pub fn principal(&self) -> Result<Principal, String> {
        let topic = self.topics.get(2).ok_or("Topic not found".to_string())?;
        let principal_trimmed: &str = topic.trim_start_matches("0x");

        let slice =
            ethers_core::utils::hex::decode(principal_trimmed).map_err(|err| err.to_string())?;

        if slice.is_empty() {
            return Err("Principal is empty".to_string());
        }

        let principal_len = (slice[0] as usize)
            .checked_sub(1)
            .unwrap_or_default()
            .div(2);
        let principal_slice = &slice[0..principal_len];

        Principal::try_from_slice(&principal_slice)
            .map_err(|err| format!("Invalid principal: {}", err.to_string()))
    }

    pub fn amount(&self) -> Result<u64, String> {
        let amount_str = self.data.trim_start_matches("0x");
        u64::from_str_radix(amount_str, 16).map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_ekoke_swapped_event() {
        let response = EkokeSwapped {
            data: "0x000000000000000000000000000000000000000000000000002386f26fc10000".to_string(),
            block_number: "0x11d8aaa".to_string(),
            topics: vec![
                "0x257e057bb61920d8d0ed2cb7b720ac7f9c513cd1110bc9fa543079154f45f435".to_string(),
                "0x00000000000000000000000053d290220b4ae5cd91987517ef04e206c1078850".to_string(),
                "0x3BCD06F8612FD5F804E0DC519CD2040571758E8FDA92A7EFEFFDF40702000000".to_string(),
            ],
        };
        assert_eq!(response.block_number().unwrap(), 18713258);
        assert_eq!(
            response.principal().unwrap(),
            Principal::from_text("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
                .unwrap()
        );
        assert_eq!(response.amount().unwrap(), 10000000000000000);
    }
}
