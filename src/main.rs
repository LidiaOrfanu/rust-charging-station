use axum::{
    routing::{get, post},
    Router, response::IntoResponse, Json,
};
use sqlx::{postgres::PgPoolOptions, query_as};
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Penguin {
    pub id: i64,
    pub name: String,
    pub species: String,
    pub age: i64
}

#[tokio::main]
async fn main()  {
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();
    assert_eq!(key, "DATABASE_URL");
    let pool = PgPoolOptions::new()
    .max_connections(100)
    .connect(&database_url)
    .await.expect("Unable to connect to Postgres");

    let select_query = query_as::<_, Penguin>("SELECT id, name, species, age FROM penguins");
	let penguins: Vec<Penguin> = select_query.fetch_all(&pool).await.unwrap();
	println!("\n=== select penguins with query.map...: \n{:?}", penguins);

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