use axum::http::Method;
use sqlx::sqlite::SqlitePoolOptions;
use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, Any};

use crate::handlers;
use crate::models::AppState;
use crate::go_api::GoApi;

pub async fn create_router() -> Router {
    let db =  SqlitePoolOptions::new()
        .max_connections(5)
        .connect("app.db")
        .await
        .expect("Failed to initialize SQLite");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let api_key = std::env::var("GO_API_KEY").expect("failed to retrieve api key");
    let api = GoApi::new(&api_key);

    let state = AppState { db, api };

    Router::new()
        .route("/api/stops", get(handlers::get_stops))
        .route("/api/stop/:stop_id", get(handlers::get_stop))
        .route("/api/stop/search/:stop_name", get(handlers::search_for_stop))
        .route("/api/route/stop/:stop_id", get(handlers::get_route_from_stop))
        .route("/api/routes", get(handlers::get_all_routes))
        .route("/api/route/:route_id", get(handlers::get_ordered_stops_for_route))
        .route("/api/route/:route_id/stop/:stop_id", get(handlers::get_stop_times))
        .with_state(state)
        .layer(cors)
}