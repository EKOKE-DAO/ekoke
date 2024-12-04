use std::str::FromStr;

use did::{HttpRequest, HttpResponse};
use ethers_core::abi::ethereum_types::H520;
use route_recognizer::Router;
use url::Url;

use crate::app::{DeferredData, SignedMessage};

const ROUTE_CONTRACTS: &str = "Contracts";
const ROUTE_CONTRACT: &str = "Contract";
const ROUTE_DOCUMENT: &str = "Document";

pub struct HttpApi;

impl HttpApi {
    /// Handles an HTTP request
    pub async fn handle_http_request(req: HttpRequest) -> HttpResponse {
        // must be a GET request
        if req.method != "GET" {
            return HttpResponse::bad_request("expected GET method".to_string());
        }

        // parse url
        let Ok(url) = Url::parse(&req.url) else {
            return HttpResponse::bad_request("invalid URL".to_string());
        };

        let mut router = Router::new();
        router.add("/contracts", ROUTE_CONTRACTS);
        router.add("/contract/:id", ROUTE_CONTRACT);
        router.add(
            "/contract/:contract_id/document/:document_id",
            ROUTE_DOCUMENT,
        );

        let Ok(route_match) = router.recognize(url.path()) else {
            return HttpResponse::not_found();
        };

        let handler = **route_match.handler();
        let params = route_match.params();
        match handler {
            ROUTE_CONTRACTS => Self::get_contracts(),

            ROUTE_CONTRACT => {
                let Some(id) = params.find("id") else {
                    return HttpResponse::bad_request("missing contract ID".to_string());
                };
                let Ok(id) = id.parse::<u64>() else {
                    return HttpResponse::bad_request("invalid contract ID".to_string());
                };
                Self::get_contract(url, id)
            }
            ROUTE_DOCUMENT => {
                let Some(contract_id) = params.find("contract_id") else {
                    return HttpResponse::bad_request("missing contract ID".to_string());
                };
                let Ok(contract_id) = contract_id.parse::<u64>() else {
                    return HttpResponse::bad_request("invalid contract ID".to_string());
                };

                let Some(document_id) = params.find("document_id") else {
                    return HttpResponse::bad_request("missing document ID".to_string());
                };
                let Ok(document_id) = document_id.parse::<u64>() else {
                    return HttpResponse::bad_request("invalid document ID".to_string());
                };

                Self::get_contract_document(url, contract_id, document_id)
            }
            _ => HttpResponse::not_found(),
        }
    }

    fn get_contracts() -> HttpResponse {
        HttpResponse::ok(DeferredData::get_contracts())
    }

    fn get_contract(url: Url, id: u64) -> HttpResponse {
        let signed_message = Self::signed_message(url);

        DeferredData::get_contract(&id.into(), signed_message)
            .map(HttpResponse::ok)
            .unwrap_or_else(HttpResponse::not_found)
    }

    fn get_contract_document(url: Url, contract_id: u64, document_id: u64) -> HttpResponse {
        let signed_message = Self::signed_message(url);

        DeferredData::get_contract_document(contract_id.into(), document_id, signed_message)
            .map(HttpResponse::ok)
            .unwrap_or_else(|_| HttpResponse::not_found())
    }

    /// Get signed message from URL
    fn signed_message(url: Url) -> Option<SignedMessage> {
        let message = Self::get_query_param(&url, "message")?;
        let signature =
            Self::get_query_param(&url, "signature").and_then(|s| H520::from_str(&s).ok())?;

        Some(SignedMessage { message, signature })
    }

    fn get_query_param(url: &Url, key: &str) -> Option<String> {
        url.query_pairs().find_map(|(k, value)| {
            if k == key {
                Some(value.to_string())
            } else {
                None
            }
        })
    }
}
