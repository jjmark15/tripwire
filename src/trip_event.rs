use std::collections::HashMap;

use chrono::{DateTime, Utc};
use getset::Getters;

#[derive(derive_new::new, Getters)]
pub(crate) struct TripEvent {
    #[getset(get = "pub")]
    timestamp: DateTime<Utc>,
    #[getset(get = "pub")]
    http_request: TripHttpRequest,
    #[getset(get = "pub")]
    request_id: String,
}

#[derive(derive_new::new, Getters)]
pub(crate) struct TripHttpRequest {
    #[getset(get = "pub")]
    body: String,
    #[getset(get = "pub")]
    headers: HashMap<String, String>,
    #[getset(get = "pub")]
    method: String,
    #[getset(get = "pub")]
    params: HashMap<String, String>,
    #[getset(get = "pub")]
    remote_address: String,
    #[getset(get = "pub")]
    uri: String,
    #[getset(get = "pub")]
    version: String,
}
