use axum::Router;

use crate::trip_state::TripState;
use crate::web::axum::axum_server;

mod trip_event;
mod trip_state;
mod web;

pub fn app() -> Router {
    axum_server(TripState::new())
}
