use axum::{
    routing::{get, post},
    Router, response::IntoResponse, Json,
};
use serde::Serialize;

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/hello", get(|| async { "Hello, World!" }));
    let app = Router::new()
    .route("/hello", get(handle_hello))
    .route("/post", post(handle_post));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_hello() -> &'static str {
    return "Hello, World!";
}

async fn handle_post() -> impl IntoResponse {
    /*
        ContentType: Application/Json
        {"id": "28isi123k"}
    */
    let data = CreatedResponse {
        id: "28isi123k".to_string()
    };
    Json(data)

}

#[derive(Serialize)]
struct CreatedResponse {
    id: String
}