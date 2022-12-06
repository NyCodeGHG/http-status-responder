use std::net::SocketAddr;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/healthz", get(|| async { StatusCode::OK }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");
    tracing::info!("Shutdown signal received, exiting");
}

async fn index<'a>() -> impl IntoResponse {
    (StatusCode::IM_A_TEAPOT, "I'm a Teapot")
}
