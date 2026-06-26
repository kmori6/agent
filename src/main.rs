use crate::presentation::http::agent::agent;
use crate::presentation::http::healthcheck::healthcheck;
use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;

mod presentation;

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/agent", post(agent));

    let app = Router::new().nest("/v1", routes);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app).await.expect("server error");
}
