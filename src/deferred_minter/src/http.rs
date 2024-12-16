mod agents;

use agents::Filters;
use candid::Principal;
use did::{HttpRequest, HttpResponse};
use route_recognizer::Router;
use url::Url;

use crate::app::DeferredMinter;

const ROUTE_AGENTS: &str = "Agents";
const ROUTE_AGENT: &str = "Agent";

pub struct HttpApi;

impl HttpApi {
    /// Handles an HTTP request
    pub async fn handle_http_request(req: HttpRequest) -> HttpResponse {
        // must be a GET request
        if req.method != "GET" {
            return HttpResponse::bad_request("expected GET method".to_string());
        }

        // convert URL to valid URL
        let url = if req.url.starts_with("/") {
            format!("http://localhost{}", req.url)
        } else {
            req.url.clone()
        };
        // parse url
        let Ok(url) = Url::parse(&url) else {
            return HttpResponse::bad_request(format!("Invalid URL: {url}"));
        };

        let mut router = Router::new();
        router.add("/agents", ROUTE_AGENTS);
        router.add("/agent/:id", ROUTE_AGENT);

        let Ok(route_match) = router.recognize(url.path()) else {
            return HttpResponse::not_found();
        };

        let handler = **route_match.handler();
        let params = route_match.params();
        match handler {
            ROUTE_AGENTS => Self::get_agencies(&url),
            ROUTE_AGENT => {
                let Some(id) = params.find("id") else {
                    return HttpResponse::bad_request("missing agent id".to_string());
                };
                let Ok(id) = Principal::from_text(id) else {
                    return HttpResponse::bad_request("invalid agent id".to_string());
                };
                Self::get_agent(id)
            }

            _ => HttpResponse::not_found(),
        }
    }

    fn get_agencies(url: &Url) -> HttpResponse {
        let filters = Filters::from(url);
        println!("Filters {:?}", filters);

        HttpResponse::ok(
            DeferredMinter::get_agencies()
                .into_iter()
                .filter(|agency| filters.check(agency))
                .collect::<Vec<_>>(),
        )
    }

    fn get_agent(id: Principal) -> HttpResponse {
        let Some(agent) = DeferredMinter::get_agent(id) else {
            return HttpResponse::not_found();
        };

        HttpResponse::ok(agent)
    }
}

#[cfg(test)]
mod test {

    use std::borrow::Cow;
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{mock_agency, with_mock_agency};
    use crate::app::Agents;

    #[tokio::test]
    async fn test_should_get_agencies() {
        let agent = mock_agency();
        Agents::insert_agency(agent.owner, agent.clone());

        let url = Url::parse("http://localhost/agents").unwrap();

        let req = HttpRequest {
            method: Cow::from("GET".to_string()),
            url: url.to_string(),
            headers: HashMap::default(),
            body: Default::default(),
        };

        let res = HttpApi::handle_http_request(req).await;
        assert_eq!(res.status_code, 200);

        let got_agents: Vec<did::deferred::Agency> = serde_json::from_slice(&res.body).unwrap();

        assert_eq!(got_agents.len(), 1);
        assert_eq!(got_agents[0], agent);
    }

    #[tokio::test]
    async fn test_should_get_agencies_with_filters() {
        let agent = with_mock_agency(|agent| {
            agent.name = "Dummy Real estate".to_string();
            agent.city = "Rome".to_string();
            agent.continent = did::deferred::Continent::Europe;
        });
        Agents::insert_agency(agent.owner, agent.clone());

        let url =
            Url::parse("http://localhost/agents?name=Dummy&city=Rome&continent=Europe").unwrap();

        let req = HttpRequest {
            method: Cow::from("GET".to_string()),
            url: url.to_string(),
            headers: HashMap::default(),
            body: Default::default(),
        };

        let res = HttpApi::handle_http_request(req).await;
        assert_eq!(res.status_code, 200);
        let got_agents: Vec<did::deferred::Agency> = serde_json::from_slice(&res.body).unwrap();

        assert_eq!(got_agents.len(), 1);
        assert_eq!(got_agents[0], agent);
    }

    #[tokio::test]
    async fn test_should_get_agent() {
        let agent = mock_agency();
        Agents::insert_agency(agent.owner, agent.clone());

        let url = Url::parse(&format!("http://localhost/agent/{}", agent.owner)).unwrap();

        let req = HttpRequest {
            method: Cow::from("GET".to_string()),
            url: url.to_string(),
            headers: HashMap::default(),
            body: Default::default(),
        };

        let res = HttpApi::handle_http_request(req).await;
        assert_eq!(res.status_code, 200);

        let got_agents: did::deferred::Agency = serde_json::from_slice(&res.body).unwrap();

        assert_eq!(got_agents, agent);
    }

    #[tokio::test]
    async fn test_should_return_not_found() {
        let url = Url::parse("http://localhost/agent/uf6dk-hyaaa-aaaaq-qaaaq-cai").unwrap();

        let req = HttpRequest {
            method: Cow::from("GET".to_string()),
            url: url.to_string(),
            headers: HashMap::default(),
            body: Default::default(),
        };

        let res = HttpApi::handle_http_request(req).await;
        assert_eq!(res.status_code, 404);
    }
}
