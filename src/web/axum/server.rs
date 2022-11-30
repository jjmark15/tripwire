use std::sync::Arc;

use axum::http::{HeaderValue, Request};
use axum::response::Response;
use axum::routing::{any, get};
use axum::Router;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::trip_state::TripState;
use crate::web::axum::echo::echo;
use crate::web::axum::trip::{trip, trip_history};

pub(crate) fn axum_server(trip_state: TripState) -> Router {
    let shared_trip_state = Arc::new(RwLock::new(trip_state));

    Router::new()
        .route("/echo/*path", any(echo))
        .route("/echo", any(echo))
        .route("/trip/*path", any(trip))
        .route("/trip", any(trip))
        .route("/history", get(trip_history))
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .map_response(add_signature)
                .map_request(add_request_id),
        )
        .with_state(shared_trip_state)
}

fn add_signature(mut response: Response) -> Response {
    tracing::debug!("adding signature headers to response");
    response
        .headers_mut()
        .insert("wizard", HeaderValue::from_str("ClumsyWizard").unwrap());
    response
        .headers_mut()
        .insert("service", HeaderValue::from_str("tripwire").unwrap());
    response
}

fn add_request_id<T>(mut req: Request<T>) -> Request<T> {
    let id = format!("tid-{}", ulid::Ulid::new().to_string());
    req.headers_mut().insert(
        "tripwire-request-id",
        HeaderValue::from_str(id.as_str()).unwrap(),
    );
    req
}
