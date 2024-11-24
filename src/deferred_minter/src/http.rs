use did::{HttpRequest, HttpResponse};

use crate::app::DeferredMinter;

pub struct HttpApi;

impl HttpApi {
    /// Handles an HTTP request
    pub async fn handle_http_request(req: HttpRequest) -> HttpResponse {
        // must be a GET request
        if req.method != "GET" {
            return HttpResponse::bad_request("expected GET method".to_string());
        }

        match req.url.as_str() {
            "/agents" => Self::get_agencies(),
            _ => HttpResponse::not_found(),
        }
    }

    fn get_agencies() -> HttpResponse {
        HttpResponse::ok(DeferredMinter::get_agencies())
    }
}
