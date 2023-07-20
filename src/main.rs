use axum::{
    routing::{get, post},
    Router,
};

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

async fn handle_post() -> &'static str {
    return "Bad post";
}
