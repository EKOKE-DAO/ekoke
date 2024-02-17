use std::borrow::Cow;
use std::collections::HashMap;

use candid::{Encode, Principal};
use did::{HttpRequest, HttpResponse};
use serde_bytes::ByteBuf;

use crate::actor::admin;
use crate::TestEnv;

pub struct HttpClient<'a> {
    principal: Principal,
    env: &'a TestEnv,
}

impl<'a> HttpClient<'a> {
    pub fn new(principal: Principal, env: &'a TestEnv) -> Self {
        Self { principal, env }
    }

    pub fn http_request<S>(&self, method: &str, params: serde_json::Value) -> S
    where
        S: serde::de::DeserializeOwned,
    {
        let mut headers: HashMap<Cow<'static, str>, Cow<'static, str>> = Default::default();
        headers.insert(
            Cow::Borrowed("content-type"),
            Cow::Borrowed("application/json"),
        );

        let body = serde_json::json!({
            "method": method,
            "params": params,
        });

        let request = HttpRequest {
            method: Cow::Borrowed("GET"),
            url: "http://localhost:8000".to_string(),
            headers,
            body: ByteBuf::from(body.to_string()),
        };

        let response: HttpResponse = self
            .env
            .query(
                self.principal,
                admin(),
                "http_request",
                Encode!(&request).unwrap(),
            )
            .unwrap();

        serde_json::from_slice(&response.body).unwrap()
    }
}
