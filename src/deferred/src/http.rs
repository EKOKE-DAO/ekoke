mod get_agencies;
mod get_contract;
mod get_token;

use did::{HttpRequest, HttpResponse};

use self::get_contract::{GetContractRequest, GetContractResponse};
use self::get_token::{GetTokenRequest, GetTokenResponse};
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
        let response = get_contract::GetContractsResponse::from(Deferred::get_signed_contracts());

        HttpResponse::ok(response)
    }

    fn get_contract(req: HttpRequest) -> HttpResponse {
        let params = match req.decode_body::<GetContractRequest>() {
            Ok(request) => request,
            Err(response) => return response,
        };
        let response = GetContractResponse::from(Deferred::get_contract(&params.id));

        HttpResponse::ok(response)
    }

    fn get_token(req: HttpRequest) -> HttpResponse {
        let params = match req.decode_body::<GetTokenRequest>() {
            Ok(request) => request,
            Err(response) => return response,
        };
        let response = GetTokenResponse::from(Deferred::get_token(&params.id));

        HttpResponse::ok(response)
    }

    fn get_agencies() -> HttpResponse {
        let response = get_agencies::GetAgenciesResponse::from(Deferred::get_agencies());

        HttpResponse::ok(response)
    }
}
