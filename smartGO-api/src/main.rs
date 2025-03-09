mod route;
mod handlers;
mod models;
mod ext_api;
mod db_provider;

pub use crate::ext_api::go_api;

use route::create_router;

#[tokio::main]
async fn main() {
    // logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // load GO_API_KEY from .env if exists (for local dev)
    let _ = dotenvy::dotenv();
    let app: axum::Router = create_router().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.expect("failed to bind address");
    axum::serve(listener, app).await.expect("failed to start server");
}