use std::net::SocketAddr;

use tripwire::app;

#[tokio::main(worker_threads = 1)]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
