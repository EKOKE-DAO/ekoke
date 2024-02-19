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
        let principal_str = self.topics.get(2).ok_or("missing principal")?;
        let principal_trimmed: &str = principal_str.trim_start_matches("0x");

        let slice =
            ethers_core::utils::hex::decode(principal_trimmed).map_err(|err| err.to_string())?;

        const ANONYMOUS_PRINCIPAL_BYTES: [u8; 1] = [4];

        if slice.is_empty() {
            return Err("slice too short".to_string());
        }
        if slice.len() > 32 {
            return Err(format!("Expected at most 32 bytes, got {}", slice.len()));
        }
        let num_bytes = slice[0] as usize;
        if num_bytes == 0 {
            return Err("management canister principal is not allowed".to_string());
        }
        if num_bytes > 29 {
            return Err(format!(
                "invalid number of bytes: expected a number in the range [1,29], got {num_bytes}",
            ));
        }
        if slice.len() < 1 + num_bytes {
            return Err("slice too short".to_string());
        }
        let (principal_bytes, trailing_zeroes) = slice[1..].split_at(num_bytes);
        if !trailing_zeroes
            .iter()
            .all(|trailing_zero| *trailing_zero == 0)
        {
            return Err("trailing non-zero bytes".to_string());
        }
        if principal_bytes == ANONYMOUS_PRINCIPAL_BYTES {
            return Err("anonymous principal is not allowed".to_string());
        }
        Principal::try_from_slice(principal_bytes).map_err(|err| err.to_string())
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
                "0x1d7a3af512fb166ee6447759bd4e3a1c7daa4d98c0b7b8cb1fbb20b62b020000".to_string(),
            ],
        };
        assert_eq!(response.block_number().unwrap(), 18713258);
        //assert_eq!(
        //    response.from_address().unwrap(),
        //    H160::from_hex_str("0x53d290220b4ae5cd91987517ef04e206c1078850").unwrap(),
        //);
        assert_eq!(
            response.principal().unwrap(),
            Principal::from_slice(&[
                0x7a, 0x3a, 0xf5, 0x12, 0xfb, 0x16, 0x6e, 0xe6, 0x44, 0x77, 0x59, 0xbd, 0x4e, 0x3a,
                0x1c, 0x7d, 0xaa, 0x4d, 0x98, 0xc0, 0xb7, 0xb8, 0xcb, 0x1f, 0xbb, 0x20, 0xb6, 0x2b,
                0x02
            ])
        );
        assert_eq!(response.amount().unwrap(), 10000000000000000);
    }
}
