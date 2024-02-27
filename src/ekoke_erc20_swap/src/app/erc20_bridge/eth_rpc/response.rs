use did::ekoke::EkokeError;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct EthRpcResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    pub error: Option<Error>,
}

impl EthRpcResponse {
    #[allow(dead_code)]
    pub fn into_result(self) -> Result<String, Error> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(self.result.unwrap()),
        }
    }
}

impl From<Error> for EkokeError {
    fn from(error: Error) -> Self {
        EkokeError::EthRpcError(error.code, error.message)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_deserialize_eth_rpc_response() {
        let json = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": "0x613bbe6a28d9ecd3ff278a60b476f5daac096e28164efb749bea47dcc11a57bf"
        }"#;

        let response: EthRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(
            response.result,
            Some("0x613bbe6a28d9ecd3ff278a60b476f5daac096e28164efb749bea47dcc11a57bf".to_string())
        );
        assert_eq!(response.error, None);

        let json = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "error": {"code":-32000,"message":"nonce too low: next nonce 24, tx nonce 0"}
        }"#;

        let response: EthRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.result, None);
        assert_eq!(
            response.error,
            Some(Error {
                code: -32000,
                message: "nonce too low: next nonce 24, tx nonce 0".to_string()
            })
        );
    }

    #[test]
    fn test_should_convert_response_into_result() {
        let json = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": "0x613bbe6a28d9ecd3ff278a60b476f5daac096e28164efb749bea47dcc11a57bf"
        }"#;

        let response: EthRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(
            response.into_result(),
            Ok("0x613bbe6a28d9ecd3ff278a60b476f5daac096e28164efb749bea47dcc11a57bf".to_string())
        );

        let json = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "error": {"code":-32000,"message":"nonce too low: next nonce 24, tx nonce 0"}
        }"#;

        let response: EthRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(
            response.into_result(),
            Err(Error {
                code: -32000,
                message: "nonce too low: next nonce 24, tx nonce 0".to_string()
            })
        );
    }
}
