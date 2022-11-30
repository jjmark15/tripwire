use std::net::SocketAddr;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use tripwire::app;

#[tokio::main(worker_threads = 1)]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(get_log_levels()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app().into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

fn get_log_levels() -> String {
    let from_env = std::env::var("TRIPWIRE_LOG").unwrap_or_else(|_| "".to_string());
    format!("info,{from_env}")
}
