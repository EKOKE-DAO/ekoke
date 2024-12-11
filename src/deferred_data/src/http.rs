mod contract_filter;

use std::str::FromStr;

use did::{HttpRequest, HttpResponse};
use ethers_core::abi::ethereum_types::H520;
use route_recognizer::Router;
use url::Url;

use self::contract_filter::Filters;
use crate::app::{ContractStorage, DeferredData, SignedMessage};

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
            ROUTE_CONTRACTS => Self::get_contracts(&url),
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

    fn get_contracts(url: &Url) -> HttpResponse {
        let filters = Filters::from(url);

        HttpResponse::ok(ContractStorage::get_contracts_filter(|contract| {
            filters.check(contract)
        }))
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

#[cfg(test)]
mod test {

    use std::borrow::Cow;
    use std::collections::HashMap;

    use candid::{Nat, Principal};
    use did::deferred::{ContractDocument, GenericValue, RestrictionLevel, Seller};
    use did::H160;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{store_mock_contract, store_mock_contract_with};

    #[tokio::test]
    async fn test_should_get_contract() {
        store_mock_contract_with(1u64, 100u64, |contract| {
            contract.restricted_properties = vec![];
        });

        let url = Url::parse("http://localhost/contract/1").unwrap();

        let req = HttpRequest {
            method: Cow::from("GET".to_string()),
            url: url.to_string(),
            headers: HashMap::default(),
            body: Default::default(),
        };

        let res = HttpApi::handle_http_request(req).await;
        assert_eq!(res.status_code, 200);

        let contract: did::deferred::Contract = serde_json::from_slice(&res.body).unwrap();

        let contract_from_storage = ContractStorage::get_contract(&Nat::from(1u64)).unwrap();
        assert_eq!(contract, contract_from_storage);
    }

    #[tokio::test]
    async fn test_should_not_get_contract() {
        let url = Url::parse("http://localhost/contract/2").unwrap();

        let req = HttpRequest {
            method: Cow::from("GET".to_string()),
            url: url.to_string(),
            headers: HashMap::default(),
            body: Default::default(),
        };

        let res = HttpApi::handle_http_request(req).await;
        assert_eq!(res.status_code, 404);
    }

    #[tokio::test]
    async fn test_should_get_contract_document() {
        store_mock_contract(1u64, 100u64);
        let document_id = ContractStorage::upload_contract_document(
            &Nat::from(1u64),
            ContractDocument {
                access_list: vec![RestrictionLevel::Public],
                mime_type: "application/pdf".to_string(),
                name: "document".to_string(),
                size: 4,
            },
            vec![0x01, 0x02, 0x03, 0x04],
        )
        .expect("Failed to upload document");

        let url = Url::parse(&format!(
            "http://localhost/contract/1/document/{document_id}"
        ))
        .unwrap();

        let req = HttpRequest {
            method: Cow::from("GET".to_string()),
            url: url.to_string(),
            headers: HashMap::default(),
            body: Default::default(),
        };

        let res = HttpApi::handle_http_request(req).await;
        assert_eq!(res.status_code, 200);

        let document: did::deferred::ContractDocumentData =
            serde_json::from_slice(&res.body).unwrap();

        assert_eq!(document.mime_type, "application/pdf".to_string());
        assert_eq!(document.data, vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[tokio::test]
    async fn test_should_not_get_contract_document() {
        store_mock_contract(1u64, 100u64);

        let url = Url::parse(&format!("http://localhost/contract/1/document/2")).unwrap();

        let req = HttpRequest {
            method: Cow::from("GET".to_string()),
            url: url.to_string(),
            headers: HashMap::default(),
            body: Default::default(),
        };

        let res = HttpApi::handle_http_request(req).await;
        assert_eq!(res.status_code, 404);
    }

    #[tokio::test]
    async fn test_should_filter_contract() {
        // total price is 100 * 100
        let min_price = 100 * 50u64;
        let max_price = 100 * 150u64;

        store_mock_contract_with(1u64, 100, |contract| {
            // insert all properties
            contract.value = 100 * 100;
            contract.buyers =
                vec![H160::from_hex_str("0x0b24F78CF0033FAbf1977D9aA61f583fBF7586D9").unwrap()];
            contract.sellers = vec![Seller {
                address: H160::from_hex_str("0x253553366da8546fc250f225fe3d25d0c782303b").unwrap(),
                quota: 100,
            }];
            contract.agency.as_mut().unwrap().owner =
                Principal::from_text("v5vof-zqaaa-aaaal-ai5cq-cai")
                    .expect("Failed to create principal");

            // insert properties
            contract.properties = vec![
                (
                    "contract:name".to_string(),
                    GenericValue::TextContent("name".to_string()),
                ),
                (
                    "contract:description".to_string(),
                    GenericValue::TextContent("description".to_string()),
                ),
                (
                    "contract:image".to_string(),
                    GenericValue::TextContent("image".to_string()),
                ),
                (
                    "contract:address".to_string(),
                    GenericValue::TextContent("address".to_string()),
                ),
                (
                    "contract:country".to_string(),
                    GenericValue::TextContent("country".to_string()),
                ),
                (
                    "contract:continent".to_string(),
                    GenericValue::TextContent("continent".to_string()),
                ),
                (
                    "contract:region".to_string(),
                    GenericValue::TextContent("region".to_string()),
                ),
                (
                    "contract:zipCode".to_string(),
                    GenericValue::TextContent("zipCode".to_string()),
                ),
                (
                    "contract:latitude".to_string(),
                    GenericValue::TextContent("45.4642".to_string()),
                ),
                (
                    "contract:longitude".to_string(),
                    GenericValue::TextContent("9.19".to_string()),
                ),
                (
                    "contract:zone".to_string(),
                    GenericValue::TextContent("zone".to_string()),
                ),
                (
                    "contract:city".to_string(),
                    GenericValue::TextContent("city".to_string()),
                ),
                (
                    "contract:squareMeters".to_string(),
                    GenericValue::Nat64Content(123),
                ),
                ("contract:rooms".to_string(), GenericValue::Nat64Content(4)),
                (
                    "contract:bathrooms".to_string(),
                    GenericValue::Nat64Content(8),
                ),
                (
                    "contract:floors".to_string(),
                    GenericValue::Nat64Content(10),
                ),
                (
                    "contract:balconies".to_string(),
                    GenericValue::Nat64Content(2),
                ),
                (
                    "contract:garden".to_string(),
                    GenericValue::BoolContent(true),
                ),
                ("contract:pool".to_string(), GenericValue::BoolContent(true)),
                (
                    "contract:garage".to_string(),
                    GenericValue::BoolContent(true),
                ),
                (
                    "contract:parking".to_string(),
                    GenericValue::BoolContent(true),
                ),
                (
                    "contract:energyClass".to_string(),
                    GenericValue::TextContent("A".to_string()),
                ),
                (
                    "contract:youtubeUrl".to_string(),
                    GenericValue::TextContent(
                        "https://www.youtube.com/watch?v=IOuTVyaNSrU".to_string(),
                    ),
                ),
            ];
        });

        // build url with filters
        let url = Url::parse(
            &format!(
                "http://localhost/contracts?latitude=45.0&longitude=9&radius=70&zone=zone&city=city&squareMeters=123&rooms=4&bathrooms=8&floors=10&balconies=2&garden=true&pool=true&garage=true&parking=true&energyClass=A&youtubeUrl=https://www.youtube.com/watch?v=IOuTVyaNSrU&minPrice={min_price}&maxPrice={max_price}",
            )
        )
        .expect("Failed to parse URL");

        let req = HttpRequest {
            method: Cow::from("GET".to_string()),
            url: url.to_string(),
            headers: HashMap::default(),
            body: Default::default(),
        };

        let res = HttpApi::handle_http_request(req).await;
        assert_eq!(res.status_code, 200);

        let contracts: Vec<Nat> = serde_json::from_slice(&res.body).unwrap();
        assert_eq!(contracts.len(), 1);
    }
}
