use axum::{response::IntoResponse, Json};

use crate::models::charging_station::CreatedResponse;

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
