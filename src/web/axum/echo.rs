use std::collections::HashMap;

use axum::http::{HeaderMap, Method, StatusCode, Version};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub(super) async fn echo(
    body: String,
    method: Method,
    headers: HeaderMap,
    version: Version,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!(EchoBody::new(body, headers, method, version))),
    )
}

#[derive(serde::Serialize)]
struct EchoBody {
    method: String,
    headers: HashMap<String, String>,
    body: String,
    version: String,
}

impl EchoBody {
    fn new(body: String, headers: HeaderMap, method: Method, version: Version) -> Self {
        EchoBody {
            method: method.to_string(),
            headers: headers
                .iter()
                .map(|(key, val)| (key.to_string(), val.to_str().unwrap().to_string()))
                .collect(),
            body,
            version: format!("{:?}", version),
        }
    }
}
