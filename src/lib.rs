use std::net::SocketAddr;

use crate::web::axum::axum_server;

mod web;

pub async fn app() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(axum_server().into_make_service())
        .await
        .unwrap();
}
