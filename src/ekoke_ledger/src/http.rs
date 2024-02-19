use did::{HttpRequest, HttpResponse};

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
            "liquidityPoolBalance" => Self::liquidity_pool_balance().await,
            "liquidityPoolAccounts" => Self::liquidity_pool_accounts(),
            _ => HttpResponse::bad_request("unknown method".to_string()),
        }
    }

    async fn liquidity_pool_balance() -> HttpResponse {
        let response = match EkokeCanister::liquidity_pool_balance().await {
            Ok(response) => response,
            Err(_) => {
                return HttpResponse::internal_error(
                    "failed to get liquidity pool balance".to_string(),
                )
            }
        };

        HttpResponse::ok(response)
    }

    fn liquidity_pool_accounts() -> HttpResponse {
        let params = EkokeCanister::liquidity_pool_accounts();

        HttpResponse::ok(params)
    }
}
