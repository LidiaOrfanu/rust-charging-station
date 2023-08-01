use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use sqlx::query_as;
use std::sync::Arc;

use crate::{
    models::charging_station::{ChargingStation, CreateChargingStation, CreatedResponse},
    AppState,
};

pub async fn handle_hello() -> &'static str {
    return "Hello, World!";
}

pub async fn handle_post() -> impl IntoResponse {
    /*
        ContentType: Application/Json
        {"id": "28isi123k"}
    */
    let data = CreatedResponse {
        id: "28isi123k".to_string(),
    };
    Json(data)
}

pub async fn handle_get_all_stations(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    const QUERY: &str = "SELECT id, name, location, availability FROM stations";
    let stations: Vec<ChargingStation> = query_as(QUERY).fetch_all(&data.db).await.unwrap();
    println!("\n=== select stations with query.map...: \n{:?}", stations);
    ((StatusCode::OK), Json(stations))
}

pub async fn handle_post_a_station(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateChargingStation>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let availability_value = if body.availability { "TRUE" } else { "FALSE" };
    let query = format!(
        "INSERT INTO stations (name, location, availability) VALUES ('{}', '{}', {}) RETURNING *",
        body.name, body.location, availability_value
    );

    let query_result = query_as::<_, ChargingStation>(&query)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(station) => {
            let station_response = json!({
                "status": "success",
                "data": {
                    "station": {
                        "id": station.id,
                        "name": station.name,
                        "location": station.location,
                        "availability": station.availability
                    }
                }
            });
        return Ok((StatusCode::CREATED, Json(station_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = json!({
                    "status": "fail",
                    "message": "Station with that name already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}
