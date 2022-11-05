use axum::Router;

use crate::web::axum::axum_server;

mod web;

pub fn app() -> Router {
    axum_server()
}
