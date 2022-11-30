use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{ConnectInfo, Query, State};
use axum::http::{HeaderMap, Method, StatusCode, Uri, Version};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tokio::sync::RwLock;

use crate::trip_event::{TripEvent, TripHttpRequest};
use crate::trip_state::TripState;
use crate::web::axum::REQUEST_ID_HEADER;

type SharedTripState = Arc<RwLock<TripState>>;

pub(crate) async fn trip(
    State(trip_state): State<SharedTripState>,
    headers: HeaderMap,
    method: Method,
    Query(params): Query<HashMap<String, String>>,
    ConnectInfo(remote_address): ConnectInfo<SocketAddr>,
    uri: Uri,
    version: Version,
    body: String,
) -> impl IntoResponse {
    let request_id = headers
        .get(REQUEST_ID_HEADER)
        .map(|val| val.to_str().unwrap())
        .unwrap_or("")
        .to_string();
    let request_time = chrono::Utc::now();
    let http_request =
        to_trip_http_request(headers, method, params, remote_address, uri, version, body);
    {
        let mut lock = trip_state.write().await;
        lock.record(TripEvent::new(request_time, http_request, request_id));
    }
    StatusCode::OK
}

fn to_trip_http_request(
    headers: HeaderMap,
    method: Method,
    params: HashMap<String, String>,
    remote_address: SocketAddr,
    uri: Uri,
    version: Version,
    body: String,
) -> TripHttpRequest {
    TripHttpRequest::new(
        body,
        headers
            .iter()
            .filter(|(key, _)| key.as_str() != REQUEST_ID_HEADER)
            .map(|(key, val)| (key.to_string(), val.to_str().unwrap().to_string()))
            .collect(),
        method.to_string(),
        params,
        remote_address.to_string(),
        uri.to_string(),
        format!("{version:?}"),
    )
}

pub(crate) async fn trip_history(State(trip_state): State<SharedTripState>) -> impl IntoResponse {
    let ser_events: Vec<TripHistoryEventSerde>;
    {
        let lock = trip_state.read().await;
        ser_events = lock
            .events()
            .iter()
            .map(|event| {
                TripHistoryEventSerde::new(
                    *event.timestamp(),
                    event.http_request().into(),
                    event.request_id().to_string(),
                )
            })
            .collect();
    }

    (StatusCode::OK, Json(json!(ser_events)))
}

#[derive(serde::Serialize, Debug, derive_new::new)]
struct TripHistoryEventSerde {
    timestamp: chrono::DateTime<chrono::Utc>,
    http_request: TripHttpRequestSerde,
    request_id: String,
}

#[derive(serde::Serialize, Debug)]
struct TripHttpRequestSerde {
    #[serde(skip_serializing_if = "String::is_empty")]
    body: String,
    headers: HashMap<String, String>,
    method: String,
    params: HashMap<String, String>,
    remote_address: String,
    uri: String,
    version: String,
}

impl From<&TripHttpRequest> for TripHttpRequestSerde {
    fn from(from: &TripHttpRequest) -> Self {
        TripHttpRequestSerde {
            body: from.body().clone(),
            headers: from.headers().clone(),
            method: from.method().clone(),
            params: from.params().clone(),
            remote_address: from.remote_address().clone(),
            uri: from.uri().clone(),
            version: from.version().clone(),
        }
    }
}
