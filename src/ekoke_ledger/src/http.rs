use did::{HttpRequest, HttpResponse};
use icrc::icrc1::Icrc1 as _;
use num_traits::ToPrimitive;

use crate::app::EkokeCanister;

pub struct HttpApi;

impl HttpApi {
    /// Handles an HTTP request
    pub async fn handle_http_request(req: HttpRequest) -> HttpResponse {
        // must be a GET request
        if req.method != "GET" {
            return HttpResponse::bad_request("expected GET method".to_string());
        }
        // Must be a JSON-RPC request
        if req.headers.get("content-type").map(|s| s.as_ref()) != Some("application/json") {
            return HttpResponse::bad_request(
                "expected content-type: application/json".to_string(),
            );
        }
        let method = match req.decode_method() {
            Ok(request) => request,
            Err(response) => return response,
        };

        match method.as_str() {
            "icrc1Name" => Self::icrc1_name(),
            "icrc1Symbol" => Self::icrc1_symbol(),
            "icrc1Decimals" => Self::icrc1_decimals(),
            "icrc1TotalSupply" => Self::icrc1_total_supply(),
            "icrc1Fee" => Self::icrc1_fee(),
            _ => HttpResponse::bad_request("unknown method".to_string()),
        }
    }

    fn icrc1_name() -> HttpResponse {
        let response = EkokeCanister::icrc1_name();

        HttpResponse::ok(serde_json::json!({
            "name": response
        }))
    }

    fn icrc1_symbol() -> HttpResponse {
        let response = EkokeCanister::icrc1_symbol();

        HttpResponse::ok(serde_json::json!({
            "symbol": response
        }))
    }

    fn icrc1_decimals() -> HttpResponse {
        let response = EkokeCanister::icrc1_decimals();

        HttpResponse::ok(serde_json::json!({
            "decimals": response
        }))
    }

    fn icrc1_total_supply() -> HttpResponse {
        let response = EkokeCanister::icrc1_total_supply();

        HttpResponse::ok(serde_json::json!({
            "totalSupply": response.0.to_u64().unwrap_or_default()
        }))
    }

    fn icrc1_fee() -> HttpResponse {
        let response = EkokeCanister::icrc1_fee();

        HttpResponse::ok(serde_json::json!({
            "fee": response.0.to_u64().unwrap_or_default()
        }))
    }
}
