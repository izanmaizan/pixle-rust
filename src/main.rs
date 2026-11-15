use axum::{routing::get, Json, Router};
use serde_json::json;

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "app": "pixle-rust" }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let app = Router::new().route("/health", get(health));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
