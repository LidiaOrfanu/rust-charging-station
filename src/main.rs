use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, query_as};
use std::fs;

use crate::{models::charging_station::ChargingStation, controllers::charging_station::{handle_hello, handle_post}};

mod models;
mod controllers;

#[tokio::main]
async fn main()  {
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();
    assert_eq!(key, "DATABASE_URL");
    let pool = PgPoolOptions::new()
    .max_connections(100)
    .connect(&database_url)
    .await.expect("Unable to connect to Postgres");

    let select_query = query_as::<_, ChargingStation>("SELECT id, name, location, availability FROM stations");
	let stations: Vec<ChargingStation> = select_query.fetch_all(&pool).await.unwrap();
	println!("\n=== select stations with query.map...: \n{:?}", stations);

    let app = Router::new()
    .route("/hello", get(handle_hello))
    .route("/post", post(handle_post));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}



