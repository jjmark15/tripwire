use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, get};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::web::axum::echo::echo;

pub(crate) fn axum_server() -> Router {
    Router::new()
        .route("/", get(hi))
        .route("/echo/*path", any(echo))
        .route("/echo", any(echo))
        .layer(ServiceBuilder::new().map_response(add_signature))
        .layer(TraceLayer::new_for_http())
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

async fn hi() -> impl IntoResponse {
    (StatusCode::OK, "hi")
}
