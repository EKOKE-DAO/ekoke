use std::borrow::Cow;
use std::collections::HashMap;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

const HTTP_OK: u16 = 200;
const HTTP_UPGRADE: u16 = 204;
const HTTP_BAD_REQUEST: u16 = 400;
const HTTP_NOT_FOUND: u16 = 404;
const HTTP_INTERNAL_ERROR: u16 = 500;

/// A HTTP response.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    /// The HTTP status code.
    pub status_code: u16,
    /// The response header map.
    pub headers: HashMap<Cow<'static, str>, Cow<'static, str>>,
    /// The response body.
    pub body: ByteBuf,
    /// Whether the query call should be upgraded to an update call.
    pub upgrade: Option<bool>,
}

impl HttpResponse {
    pub fn new(
        status_code: u16,
        headers: HashMap<Cow<'static, str>, Cow<'static, str>>,
        body: ByteBuf,
        upgrade: Option<bool>,
    ) -> Self {
        Self {
            status_code,
            headers,
            body,
            upgrade,
        }
    }

    /// Returns a new `HttpResponse` intended to be used for internal errors.
    pub fn internal_error(e: String) -> Self {
        let body = match serde_json::to_vec(&e) {
            Ok(bytes) => ByteBuf::from(&bytes[..]),
            Err(e) => ByteBuf::from(e.to_string().as_bytes()),
        };

        Self {
            status_code: HTTP_INTERNAL_ERROR,
            headers: HashMap::from([("content-type".into(), "application/json".into())]),
            body,
            upgrade: None,
        }
    }

    /// Returns a new `HttpResponse` intended to be used for bad request
    pub fn bad_request(e: String) -> Self {
        let body = match serde_json::to_vec(&e) {
            Ok(bytes) => ByteBuf::from(&bytes[..]),
            Err(e) => ByteBuf::from(e.to_string().as_bytes()),
        };

        Self {
            status_code: HTTP_BAD_REQUEST,
            headers: HashMap::from([("content-type".into(), "application/json".into())]),
            body,
            upgrade: None,
        }
    }

    /// Returns a new `HttpResponse` intended to be used for not found
    pub fn not_found() -> Self {
        Self {
            status_code: HTTP_NOT_FOUND,
            headers: HashMap::from([("content-type".into(), "application/json".into())]),
            body: ByteBuf::from("Not Found".as_bytes()),
            upgrade: None,
        }
    }

    /// Returns an OK response with the given body.
    pub fn ok<S>(body: S) -> Self
    where
        S: Serialize,
    {
        let body = match serde_json::to_string(&body) {
            Ok(body) => body,
            Err(e) => return HttpResponse::internal_error(e.to_string()),
        };
        Self::new(
            HTTP_OK,
            HashMap::from([("content-type".into(), "application/json".into())]),
            ByteBuf::from(body.as_bytes()),
            None,
        )
    }

    /// Upgrade response to update call.
    pub fn upgrade_response() -> Self {
        Self::new(
            HTTP_UPGRADE,
            HashMap::default(),
            ByteBuf::default(),
            Some(true),
        )
    }
}

/// The important components of an HTTP request.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    /// The HTTP method string.
    pub method: Cow<'static, str>,
    /// The URL that was visited.
    pub url: String,
    /// The request headers.
    pub headers: HashMap<Cow<'static, str>, Cow<'static, str>>,
    /// The request body.
    pub body: ByteBuf,
}

impl HttpRequest {
    pub fn new(data: &[u8]) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".into(), "application/json".into());
        Self {
            method: "POST".into(),
            url: "".into(),
            headers,
            body: ByteBuf::from(data),
        }
    }

    pub fn decode_body<S>(&self) -> Result<S, HttpResponse>
    where
        S: serde::de::DeserializeOwned,
    {
        serde_json::from_slice::<HttpApiRequest<S>>(&self.body)
            .map_err(|_| HttpResponse::bad_request("Invalid request body".to_string()))
            .map(|m| m.params)
    }

    pub fn decode_method(&self) -> Result<String, HttpResponse> {
        serde_json::from_slice::<HttpApiMethod>(&self.body)
            .map_err(|_| HttpResponse::bad_request("Invalid request body".to_string()))
            .map(|m| m.method)
    }
}

#[derive(Clone, Debug, Deserialize)]
struct HttpApiMethod {
    pub method: String,
}

/// The important components of an HTTP request.
#[derive(Clone, Debug, Deserialize)]
pub struct HttpApiRequest<S> {
    pub method: String,
    pub params: S,
}
