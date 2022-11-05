use axum::http::{HeaderValue, Request, StatusCode};
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
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .map_response(add_signature)
                .map_request(add_request_id),
        )
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

async fn hi() -> impl IntoResponse {
    (StatusCode::OK, "hi")
}
