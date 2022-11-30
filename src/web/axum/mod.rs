pub(crate) use server::axum_server;

mod echo;
mod server;
mod trip;

const REQUEST_ID_HEADER: &str = "tripwire-request-id";
