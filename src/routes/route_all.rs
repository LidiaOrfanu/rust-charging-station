use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{controllers::charging_station::{handle_hello, handle_post, handle_get_all_stations}, AppState};

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/hello", get(handle_hello))
        .route("/post", post(handle_post))
        .route("/stations", get(handle_get_all_stations))
        .with_state(app_state)
}