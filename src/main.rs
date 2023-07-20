use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/hello", get(|| async { "Hello, World!" }));
    let app = Router::new().route("/hello", get(handleHello));


    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handleHello() -> &'static str {
    return "Hello, World!";
}
7