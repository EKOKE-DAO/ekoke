mod get_contract;
mod get_token;

use did::{HttpRequest, HttpResponse};

use self::get_contract::GetContractRequest;
use self::get_token::GetTokenRequest;
use crate::app::Deferred;

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
            "getContracts" => Self::get_contracts(),
            "getContract" => Self::get_contract(req),
            "getToken" => Self::get_token(req),
            "getAgencies" => Self::get_agencies(),
            _ => HttpResponse::bad_request("unknown method".to_string()),
        }
    }

    fn get_contracts() -> HttpResponse {
        HttpResponse::ok(Deferred::get_signed_contracts())
    }

    fn get_contract(req: HttpRequest) -> HttpResponse {
        let params = match req.decode_body::<GetContractRequest>() {
            Ok(request) => request,
            Err(response) => return response,
        };
        Deferred::get_contract(&params.id)
            .map(HttpResponse::ok)
            .unwrap_or_else(HttpResponse::not_found)
    }

    fn get_token(req: HttpRequest) -> HttpResponse {
        let params = match req.decode_body::<GetTokenRequest>() {
            Ok(request) => request,
            Err(response) => return response,
        };
        Deferred::get_token(&params.id)
            .map(HttpResponse::ok)
            .unwrap_or_else(HttpResponse::not_found)
    }

    fn get_agencies() -> HttpResponse {
        HttpResponse::ok(Deferred::get_agencies())
    }
}
