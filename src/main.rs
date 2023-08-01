use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{fs, sync::Arc};

use crate::routes::route_all::routes;
mod controllers;
mod models;
mod routes;

pub struct AppState {
    db: Pool<Postgres>,
}
#[tokio::main]
async fn main() {
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();
    assert_eq!(key, "DATABASE_URL");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            print!("🦀 Succesfull connection to the database");
            pool
        }
        Err(err) => {
            println!("💣 Failed to connect: {err}");
            std::process::exit(1);
        }
    };

    let app = routes(Arc::new(AppState { db: pool.clone() }));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
