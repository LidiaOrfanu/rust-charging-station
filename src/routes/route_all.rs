use axum::{Router, routing::{get, post}};

use crate::controllers::charging_station::{handle_hello, handle_post};

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handle_hello))
        .route("/post", post(handle_post))
}