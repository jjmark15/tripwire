use std::collections::HashMap;
use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Query};
use axum::http::{HeaderMap, Method, StatusCode, Uri, Version};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub(super) async fn echo(
    headers: HeaderMap,
    method: Method,
    Query(params): Query<HashMap<String, String>>,
    ConnectInfo(remote_address): ConnectInfo<SocketAddr>,
    uri: Uri,
    version: Version,
    body: String,
) -> impl IntoResponse {
    let echo = EchoBody::new(body, headers, method, params, remote_address, uri, version);
    tracing::debug!("echo response body: {echo:#?}");
    (StatusCode::OK, Json(json!((echo))))
}

#[derive(serde::Serialize, Debug)]
struct EchoBody {
    #[serde(skip_serializing_if = "String::is_empty")]
    body: String,
    headers: HashMap<String, String>,
    method: String,
    params: HashMap<String, String>,
    remote_address: String,
    uri: String,
    version: String,
}

impl EchoBody {
    fn new(
        body: String,
        headers: HeaderMap,
        method: Method,
        params: HashMap<String, String>,
        remote_address: SocketAddr,
        uri: Uri,
        version: Version,
    ) -> Self {
        EchoBody {
            body,
            headers: headers
                .iter()
                .filter(|(key, _)| key.as_str() != "tripwire-request-id")
                .map(|(key, val)| (key.to_string(), val.to_str().unwrap().to_string()))
                .collect(),
            method: method.to_string(),
            params,
            remote_address: remote_address.to_string(),
            uri: uri.to_string(),
            version: format!("{version:?}"),
        }
    }
}
