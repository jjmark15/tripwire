use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub(crate) fn axum_server() -> Router {
    Router::new().route("/", get(hi))
}

async fn hi() -> impl IntoResponse {
    (StatusCode::OK, "hi")
}
