use std::sync::Arc;
use axum::{response::IntoResponse, Json, http::StatusCode, extract::State};
use sqlx::query_as;

use crate::{models::charging_station::{CreatedResponse, ChargingStation}, AppState};

pub async fn handle_hello() -> &'static str {
    return "Hello, World!";
}

pub async fn handle_post() -> impl IntoResponse {
    /*
        ContentType: Application/Json
        {"id": "28isi123k"}
    */
    let data = CreatedResponse {
        id: "28isi123k".to_string()
    };
    Json(data)

}

pub async fn handle_get_all_stations(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    const QUERY: &str = "SELECT id, name, location, availability FROM stations";
	let stations: Vec<ChargingStation> = query_as(QUERY).fetch_all(&data.db).await.unwrap();
	println!("\n=== select stations with query.map...: \n{:?}", stations);
    ((StatusCode::OK), Json(stations))

}