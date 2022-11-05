use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{any, get};
use axum::Router;

use crate::web::axum::echo::echo;

pub(crate) fn axum_server() -> Router {
    Router::new()
        .route("/", get(hi))
        .route("/echo/*path", any(echo))
        .route("/echo", any(echo))
}

async fn hi() -> impl IntoResponse {
    (StatusCode::OK, "hi")
}
