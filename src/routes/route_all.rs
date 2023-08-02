use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::charging_station::{
        handle_get_all_stations, handle_hello, handle_post, handle_post_a_station, handler_get_station_by_id,
    },
    AppState,
};

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/hello", get(handle_hello))
        .route("/post", post(handle_post))
        .route("/api/stations", get(handle_get_all_stations))
        .route("/api/station", post(handle_post_a_station))
        .route("/api/stations/:id", get(handler_get_station_by_id))
        .with_state(app_state)
}
