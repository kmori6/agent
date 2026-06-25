use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/v1/healthcheck", get(healthcheck));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app).await.expect("server error");
}

async fn healthcheck() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
